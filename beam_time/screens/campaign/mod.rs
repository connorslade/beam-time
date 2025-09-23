use std::f32::consts::TAU;

use ahash::{HashMap, HashMapExt};
use beam_logic::level::{Level, default::DEFAULT_LEVELS, tree::LevelTree};
use engine::{
    color::Rgb,
    drawable::{
        Anchor, Drawable,
        shape::{rectangle::Rectangle, rectangle_outline::RectangleOutline},
        sprite::Sprite,
        text::Text,
    },
    exports::{
        nalgebra::Vector2,
        winit::{event::MouseButton, window::CursorIcon},
    },
    graphics_context::GraphicsContext,
    layout::{
        LayoutElement, LayoutMethods, column::ColumnLayout, root::RootLayout,
        tracker::LayoutTracker,
    },
    memory_key,
};
use slug::slugify;
use uuid::Uuid;

use crate::{
    app::App,
    assets::{ALAGARD_FONT, CHECK, UNDEAD_FONT},
    consts::{color, keybind, paths, spacing::PADDING},
    game::{
        board::{
            Board, BoardMeta, LevelMeta,
            unloaded::{UnloadedBoard, load_level_dir},
        },
        pancam::Pancam,
    },
    ui::{components::manual_button::ManualButton, misc::title_layout, pixel_line::PixelLine},
};

use super::{Screen, game::GameScreen};

mod layout;
use layout::TreeLayout;

const SPACING: f32 = 64.0;

pub struct CampaignScreen {
    tree: LevelTree,
    layout: TreeLayout,
    pancam: Pancam,

    worlds: HashMap<Uuid, Vec<UnloadedBoard>>,
}

impl Screen for CampaignScreen {
    fn tick(&mut self, _state: &mut App, ctx: &mut GraphicsContext) {
        if self.layout.is_empty() || ctx.window.dpi_changed().is_some() {
            self.layout = TreeLayout::generate(&self.tree, ctx);
        }
    }

    fn render(&mut self, state: &mut App, ctx: &mut GraphicsContext) {
        ctx.background(color::BACKGROUND);
        let t = state.start.elapsed().as_secs_f32();

        let (scale, pos) = title_layout(ctx, 8.0);
        let title_padding = 6.0 * (scale / 3.0).round();

        let mut root = RootLayout::new(pos, Anchor::TopCenter);
        let tracker = LayoutTracker::new(memory_key!());
        ColumnLayout::new(title_padding)
            .tracked(tracker)
            .show(ctx, &mut root, |ctx, layout| {
                Text::new(ALAGARD_FONT, "Campaign")
                    .scale(Vector2::repeat(scale.round()))
                    .z_index(4)
                    .default_shadow()
                    .layout(ctx, layout);

                let percent = self.solved_count(state) as f32 / self.tree.count() as f32 * 100.0;
                Text::new(UNDEAD_FONT, format!("{percent:.0}% Complete"))
                    .scale(Vector2::repeat((scale / 3.0).round()))
                    .position(Vector2::x() * title_padding, Anchor::BottomLeft)
                    .z_index(4)
                    .default_shadow()
                    .layout(ctx, layout);
            });

        root.draw(ctx);

        if let Some(bounds) = tracker.bounds(ctx) {
            Rectangle::new(bounds.size() + Vector2::repeat(PADDING * 2.0))
                .position(bounds.min - Vector2::repeat(PADDING), Anchor::BottomLeft)
                .color(color::BACKGROUND)
                .z_index(3)
                .draw(ctx);
        }

        self.pancam.update(state, ctx);

        ctx.input
            .key_pressed(keybind::BACK)
            .then(|| state.pop_screen());

        let center = ctx.center();
        let height = self.layout.rows[0][0].height;
        let origin = Vector2::new(
            center.x,
            (center.y - (height as f32 * SPACING) / 2.0).max(SPACING),
        );

        for (i, row) in self.layout.rows.iter().enumerate() {
            let offset = origin + Vector2::y() * i as f32 * SPACING;

            for (j, item) in row.iter().enumerate() {
                let available = self.is_available(state, item.id) || state.config.debug;
                let worlds = self.worlds.get(&item.id);
                let solved = worlds
                    .map(|x| x.iter().any(|x| x.meta.is_solved()))
                    .unwrap_or_default();
                let ever_solved = state.level_solved(&item.id);

                let center = offset + Vector2::x() * item.offset();
                let text = item.text.clone();
                let text = text
                    .position(self.pancam.pan + center, Anchor::Center)
                    .z_index(1)
                    .color([Rgb::repeat(0.8), Rgb::repeat(1.0)][available as usize])
                    .default_shadow();
                let hover = text.is_hovered(ctx);
                ManualButton::new(memory_key!(i, j))
                    .hovered(hover && available)
                    .tick(ctx);

                if solved {
                    let size = text.size(ctx);
                    let offset = Vector2::new(size.x / 2.0 + 9.0, size.y / 2.0);
                    Sprite::new(CHECK)
                        .position(self.pancam.pan + center + offset, Anchor::Center)
                        .scale(Vector2::repeat(3.0))
                        .z_index(3)
                        .draw(ctx);
                }

                if hover {
                    ctx.window
                        .cursor([CursorIcon::NotAllowed, CursorIcon::Pointer][available as usize]);

                    if available {
                        let size = text.size(ctx);
                        RectangleOutline::new(Vector2::new(size.x + 8.0, size.y + 8.0), 2.0)
                            .position(self.pancam.pan + center, Anchor::Center)
                            .z_index(2)
                            .draw(ctx);

                        if ctx.input.mouse_pressed(MouseButton::Left) {
                            // We can remove it bc we're going to be going to a new
                            // screen and dropping this anyway
                            let worlds = self.worlds.remove(&item.id);
                            self.open_level(state, worlds, self.tree.get(item.id).unwrap());
                        }
                    }
                }

                text.draw(ctx);

                for child in item.children.iter() {
                    let offset = self.layout.rows[i + 1][*child].offset();
                    let (_, shapes) = ctx.draw_callback(|ctx| {
                        let end = origin + Vector2::new(offset, (i + 1) as f32 * SPACING);
                        let mid = (center + end) / 2.0;

                        // todo: like optimize this or smth
                        PixelLine::new(center, Vector2::new(center.x, mid.y))
                            .color(Rgb::repeat(0.6))
                            .position(self.pancam.pan)
                            .draw(ctx);
                        PixelLine::new(Vector2::new(center.x, mid.y), Vector2::new(end.x, mid.y))
                            .color(Rgb::repeat(0.6))
                            .position(self.pancam.pan)
                            .draw(ctx);
                        PixelLine::new(Vector2::new(end.x, mid.y), end)
                            .color(Rgb::repeat(0.6))
                            .position(self.pancam.pan)
                            .draw(ctx);
                    });

                    if !ever_solved && !solved {
                        continue;
                    }

                    // epic laser beam effect
                    for (idx, shape) in shapes.chunks_mut(4).enumerate() {
                        let frac = (idx as f32 / 50.0 * TAU * 3.0 - t * 5.0).sin() / 2.0 + 0.5;
                        let color = Rgb::hex(0xe43636).lerp(Rgb::repeat(0.0), frac * 0.5);
                        shape.iter_mut().for_each(|x| x.color = color);
                    }
                }
            }
        }
    }

    fn on_init(&mut self, state: &mut App) {
        self.worlds.clear();

        let campaign = state.data_dir.join(paths::CAMPAIGN);
        if !campaign.exists() {
            return;
        }

        for board in load_level_dir(&campaign) {
            let Some(level) = &board.meta.level else {
                continue;
            };
            self.worlds.entry(level.id).or_default().push(board);
        }

        #[cfg(feature = "steam")]
        if self.all_solved() {
            state.steam.award_achievement("campaign_complete");
        }
    }
}

impl CampaignScreen {
    pub fn open_level(
        &self,
        state: &mut App,
        solutions: Option<Vec<UnloadedBoard>>,
        level: &Level,
    ) {
        let latest = solutions
            .as_ref()
            .and_then(|x| x.iter().max_by_key(|x| x.meta.last_played));

        if let Some(UnloadedBoard { path, .. }) = latest {
            state
                .push_screen(GameScreen::load(path).with_solutions(solutions.unwrap().into_iter()));
        } else {
            let board = Board {
                meta: BoardMeta {
                    name: "New Solution 1".into(),
                    level: Some(LevelMeta {
                        id: level.id,
                        solved: None,
                    }),
                    size: level.size,
                    ..Default::default()
                },
                tiles: level.tiles.clone(),
                ..Default::default()
            };

            let id = Uuid::new_v4();
            let path = state
                .data_dir
                .join(paths::CAMPAIGN)
                .join(format!("{}_{id}.bin", slugify(&level.name)));
            state.push_screen(GameScreen::new(board, path));
        }
    }

    fn is_available(&self, state: &App, id: Uuid) -> bool {
        let Some(parents) = self.tree.parents(id) else {
            return true;
        };

        for id in parents {
            let worlds = self.worlds.get(id);
            if worlds
                .map(|x| x.iter().any(|x| x.ever_solved(state)))
                .unwrap_or_default()
            {
                return true;
            }
        }

        parents.is_empty()
    }

    fn solved_count(&self, state: &App) -> usize {
        self.worlds
            .values()
            .filter(|x| x.iter().any(|x| x.ever_solved(state)))
            .count()
    }

    #[cfg(feature = "steam")]
    fn all_solved(&self) -> bool {
        for level in DEFAULT_LEVELS.iter() {
            let Some(level) = self.worlds.get(&level.id) else {
                return false;
            };

            if !level.iter().any(|x| x.meta.is_solved()) {
                return false;
            }
        }

        true
    }
}

impl Default for CampaignScreen {
    fn default() -> Self {
        let tree = LevelTree::new(&DEFAULT_LEVELS);
        Self {
            tree,
            layout: TreeLayout::default(),
            pancam: Pancam::default().with_zoom_sensitivity(0.0),

            worlds: HashMap::new(),
        }
    }
}
