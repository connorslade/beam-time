use std::path::PathBuf;

use ahash::HashMap;
use beam_logic::level::{Level, DEFAULT_LEVELS};
use common::misc::in_bounds;
use engine::{
    color::Rgb,
    drawable::text::Text,
    exports::{nalgebra::Vector2, winit::event::MouseButton},
    graphics_context::{Anchor, GraphicsContext},
    screens::Screen,
};
use log::warn;
use uuid::Uuid;

use crate::{
    app::App,
    assets::UNDEAD_FONT,
    game::board::{Board, BoardMeta, LevelMeta},
    screens::game::GameScreen,
    ui::{
        button::ButtonState,
        misc::{font_scale, titled_screen},
    },
    util::load_level_dir,
};

#[derive(Default)]
pub struct CampaignScreen {
    back_button: ButtonState,

    runtime_levels: Vec<Level>,
    worlds: HashMap<Uuid, (PathBuf, BoardMeta)>,
}

impl Screen<App> for CampaignScreen {
    fn render(&mut self, state: &mut App, ctx: &mut GraphicsContext<App>) {
        titled_screen(state, ctx, Some(&mut self.back_button), "Campaign");

        const SCALE: f32 = 3.0;
        let (_line_height, line_spacing, total_height) =
            font_scale(ctx, UNDEAD_FONT, SCALE, DEFAULT_LEVELS.len());

        for (i, level) in DEFAULT_LEVELS
            .iter()
            .chain(self.runtime_levels.iter())
            .enumerate()
        {
            let world = self.worlds.get(&level.id);
            let color = if world.map(|(_, meta)| meta.is_solved()) == Some(true) {
                Rgb::hex(0x8fd032)
            } else {
                Rgb::new(1.0, 1.0, 1.0)
            };

            let pos =
                ctx.center() + Vector2::new(0.0, total_height / 2.0 - line_spacing * i as f32);

            let mut text = Text::new(UNDEAD_FONT, &level.name)
                .position(pos, Anchor::Center)
                .scale(Vector2::repeat(SCALE))
                .color(color);

            let half_size = text.size(ctx) / 2.0;
            let hovered = in_bounds(ctx.input.mouse, (pos - half_size, pos + half_size));
            if hovered {
                text = text.color(color.lerp(Rgb::hex(0), 0.25));

                if ctx.input.mouse_pressed(MouseButton::Left) {
                    if let Some((path, _meta)) = world {
                        ctx.push_screen(GameScreen::load(path.to_path_buf()));
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
                        ctx.push_screen(GameScreen::new(board, path));
                    }
                }
            }

            ctx.draw(text);
        }
    }

    fn on_init(&mut self, state: &mut App) {
        self.worlds.clear();
        self.runtime_levels.clear();

        let campaign = state.data_dir.join("campaign");
        if campaign.exists() {
            for (path, meta) in load_level_dir(&campaign) {
                let Some(level) = meta.level else { continue };
                self.worlds.insert(level.id, (path, meta));
            }
        }

        let levels = state.data_dir.join("levels");
        if levels.exists() {
            for path in levels.read_dir().unwrap().filter_map(|x| x.ok()) {
                let level = match Level::load_file(path.path()) {
                    Ok(x) => x,
                    Err(e) => {
                        warn!("Error loading custom level: {e}");
                        continue;
                    }
                };

                self.runtime_levels.push(level);
            }
        }
    }
}
