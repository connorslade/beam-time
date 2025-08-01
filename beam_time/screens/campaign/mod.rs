use std::{f32::consts::TAU, path::PathBuf};

use ahash::{HashMap, HashMapExt};
use beam_logic::level::{DEFAULT_LEVELS, Level, tree::LevelTree};
use engine::{
    color::Rgb,
    drawable::{shape::rectangle_outline::RectangleOutline, sprite::Sprite},
    exports::{
        nalgebra::Vector2,
        winit::{event::MouseButton, keyboard::KeyCode, window::CursorIcon},
    },
    graphics_context::{Anchor, Drawable, GraphicsContext},
};
use uuid::Uuid;

use crate::{
    app::App,
    assets::CHECK,
    consts::BACKGROUND_COLOR,
    game::{
        board::{Board, BoardMeta, LevelMeta},
        pancam::Pancam,
    },
    ui::pixel_line::PixelLine,
    util::load_level_dir,
};

use super::{Screen, game::GameScreen};

mod layout;
use layout::TreeLayout;

pub struct CampaignScreen {
    tree: LevelTree,
    layout: TreeLayout,
    pancam: Pancam,

    worlds: HashMap<Uuid, (PathBuf, BoardMeta)>,
}

impl Screen for CampaignScreen {
    fn render(&mut self, state: &mut App, ctx: &mut GraphicsContext) {
        ctx.background(BACKGROUND_COLOR);
        let t = state.start.elapsed().as_secs_f32();

        self.pancam.update(state, ctx);
        let spacing = 64.0 * ctx.scale_factor;

        ctx.input
            .key_pressed(KeyCode::Escape)
            .then(|| state.pop_screen());

        if self.layout.is_empty() || ctx.input.dpi_changed() {
            self.layout = TreeLayout::generate(&self.tree, ctx);
        }

        for (i, row) in self.layout.rows.iter().enumerate() {
            let offset = Vector2::y() * i as f32 * spacing;

            for item in row {
                let world = self.worlds.get(&item.id);
                let solved = world.map(|(_, meta)| meta.is_solved()) == Some(true);

                let center = offset + Vector2::x() * item.offset() + ctx.center();
                let text = item
                    .text
                    .clone()
                    .position(self.pancam.pan + center, Anchor::Center)
                    .z_index(1)
                    .default_shadow();

                if solved {
                    let size = text.size(ctx);
                    let offset = Vector2::new(size.x / 2.0 + 9.0 * ctx.scale_factor, size.y / 2.0);
                    Sprite::new(CHECK)
                        .position(self.pancam.pan + center + offset, Anchor::Center)
                        .scale(Vector2::repeat(3.0))
                        .z_index(3)
                        .draw(ctx);
                }

                if text.is_hovered(ctx) {
                    let size = text.size(ctx);
                    let px = 2.0 * ctx.scale_factor;
                    ctx.set_cursor(CursorIcon::Pointer);
                    RectangleOutline::new(Vector2::new(size.x + px * 4.0, size.y + px * 4.0), 2.0)
                        .position(self.pancam.pan + center, Anchor::Center)
                        .z_index(2)
                        .draw(ctx);

                    // todo: invalidate if dragged after mouse down
                    if ctx.input.mouse_released(MouseButton::Left) {
                        self.open_level(state, world, self.tree.get(item.id).unwrap());
                    }
                }

                text.draw(ctx);

                for child in item.children.iter() {
                    let offset = self.layout.rows[i + 1][*child].offset();
                    let (_, shapes) = ctx.draw_callback(|ctx| {
                        let end = Vector2::new(offset, (i + 1) as f32 * spacing) + ctx.center();
                        PixelLine::new(center, end)
                            .color(Rgb::repeat(0.6))
                            .position(self.pancam.pan)
                            .draw(ctx)
                    });

                    if !solved {
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

        let campaign = state.data_dir.join("campaign");
        if !campaign.exists() {
            return;
        }

        for (path, meta) in load_level_dir(&campaign) {
            let Some(level) = meta.level else { continue };
            self.worlds.insert(level.id, (path, meta));
        }

        #[cfg(feature = "steam")]
        if self.all_solved() {
            state.steam.award_achievement("campaign_complete");
        }
    }
}

impl CampaignScreen {
    pub fn open_level(&self, state: &mut App, world: Option<&(PathBuf, BoardMeta)>, level: &Level) {
        if let Some((path, _meta)) = world {
            state.push_screen(GameScreen::load(path.to_path_buf()));
        } else {
            let board = Board {
                meta: BoardMeta {
                    name: level.name.to_owned(),
                    level: Some(LevelMeta {
                        id: level.id,
                        solved: false,
                    }),
                    size: level.size,
                    ..Default::default()
                },
                tiles: level.tiles.clone(),
                ..Default::default()
            };
            let path = state
                .data_dir
                .join("campaign")
                .join(level.id.to_string())
                .with_extension("bin");
            state.push_screen(GameScreen::new(board, path));
        }
    }

    #[cfg(feature = "steam")]
    pub fn all_solved(&self) -> bool {
        for level in self.worlds.values() {
            if !level.1.is_solved() {
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
            pancam: Pancam::default(),

            worlds: HashMap::new(),
        }
    }
}
