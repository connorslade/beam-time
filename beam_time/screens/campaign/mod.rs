use std::path::PathBuf;

use ahash::{HashMap, HashMapExt};
use beam_logic::level::{tree::LevelTree, Level, DEFAULT_LEVELS};
use engine::{
    color::Rgb,
    drawable::shape::rectangle_outline::RectangleOutline,
    exports::{
        nalgebra::Vector2,
        winit::{event::MouseButton, keyboard::KeyCode, window::CursorIcon},
    },
    graphics_context::{Anchor, Drawable, GraphicsContext},
};
use uuid::Uuid;

use crate::{
    app::App,
    consts::BACKGROUND_COLOR,
    game::{
        board::{Board, BoardMeta, LevelMeta},
        pancam::Pancam,
    },
    ui::pixel_line::PixelLine,
    util::load_level_dir,
};

use super::{game::GameScreen, Screen};

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
                let color = if world.map(|(_, meta)| meta.is_solved()) == Some(true) {
                    Rgb::hex(0x8fd032)
                } else {
                    Rgb::hex(0xFFFFFF)
                };

                let center = offset + Vector2::x() * item.offset();
                let text = item
                    .text
                    .clone()
                    .position(self.pancam.pan + center, Anchor::Center)
                    .color(color)
                    .z_index(1);

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
                    PixelLine::new(center, Vector2::new(offset, (i + 1) as f32 * spacing))
                        .color(color.lerp(Rgb::repeat(0.0), 0.6))
                        .position(self.pancam.pan)
                        .draw(ctx);
                }
            }
        }
    }

    fn on_init(&mut self, state: &mut App) {
        self.worlds.clear();

        let campaign = state.data_dir.join("campaign");
        if campaign.exists() {
            for (path, meta) in load_level_dir(&campaign) {
                let Some(level) = meta.level else { continue };
                self.worlds.insert(level.id, (path, meta));
            }
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
