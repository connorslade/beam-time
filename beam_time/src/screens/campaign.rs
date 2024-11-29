use std::{collections::HashMap, path::PathBuf};

use beam_logic::level::DEFAULT_LEVELS;
use common::misc::in_bounds;
use engine::{
    color::Rgb,
    drawable::text::Text,
    exports::{nalgebra::Vector2, winit::event::MouseButton},
    graphics_context::{Anchor, GraphicsContext},
    screens::Screen,
};
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
    worlds: HashMap<Uuid, (PathBuf, BoardMeta)>,
}

impl Screen<App> for CampaignScreen {
    fn render(&mut self, state: &mut App, ctx: &mut GraphicsContext<App>) {
        titled_screen(state, ctx, &mut self.back_button, "Campaign");

        const SCALE: f32 = 3.0;
        let (_line_height, line_spacing, total_height) =
            font_scale(ctx, UNDEAD_FONT, SCALE, DEFAULT_LEVELS.len());

        for (i, level) in DEFAULT_LEVELS.iter().enumerate() {
            let pos =
                ctx.center() + Vector2::new(0.0, total_height / 2.0 - line_spacing * i as f32);

            let mut text = Text::new(UNDEAD_FONT, &level.name)
                .position(pos, Anchor::Center)
                .scale(Vector2::repeat(SCALE));

            let half_size = text.size(ctx) / 2.0;
            let hovered = in_bounds(ctx.input.mouse, (pos - half_size, pos + half_size));
            if hovered {
                text = text.color(Rgb::new(0.5, 0.5, 0.5));

                if ctx.input.mouse_pressed(MouseButton::Left) {
                    if let Some((path, _meta)) = self.worlds.get(&level.id) {
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
        let dir = state.data_dir.join("campaign");
        if dir.exists() {
            for (path, meta) in load_level_dir(dir) {
                let Some(level) = meta.level else { continue };
                self.worlds.insert(level.id, (path, meta));
            }
        }
    }
}
