use std::{fs::File, path::PathBuf};

use anyhow::Result;
use engine::{
    color::Rgb,
    drawable::text::Text,
    exports::{nalgebra::Vector2, winit::event::MouseButton},
    graphics_context::{Anchor, GraphicsContext},
    screens::Screen,
};
use log::warn;

use crate::{
    assets::UNDEAD_FONT,
    game::board::BoardMeta,
    screens::game::GameScreen,
    ui::{button::ButtonState, misc::{font_scale, titled_screen}},
    util::in_bounds,
    App,
};

#[derive(Default)]
pub struct SandboxScreen {
    worlds: Vec<(PathBuf, BoardMeta)>,
    back_button: ButtonState,
}

impl Screen<App> for SandboxScreen {
    fn render(&mut self, state: &mut App, ctx: &mut GraphicsContext<App>) {
        titled_screen(state, ctx, &mut self.back_button, "Sandbox");

        if self.worlds.is_empty() {
            ctx.draw(
                Text::new(UNDEAD_FONT, "No worlds...")
                    .pos(ctx.center(), Anchor::Center)
                    .scale(Vector2::repeat(4.0)),
            );
        } else {
            const SCALE: f32 = 3.0;
            let (line_height, line_spacing, total_height) =
                font_scale(ctx, UNDEAD_FONT, SCALE, self.worlds.len());

            for (i, (world, meta)) in self.worlds.iter().enumerate() {
                let pos =
                    ctx.center() + Vector2::new(0.0, total_height / 2.0 - line_spacing * i as f32);

                let mut text = Text::new(UNDEAD_FONT, &meta.name)
                    .pos(pos, Anchor::Center)
                    .scale(Vector2::repeat(SCALE));

                let width = text.width(ctx) * SCALE;
                let half_size = Vector2::new(width / 2.0, line_height / 2.0) * ctx.scale_factor;
                let hovered = in_bounds(ctx.input.mouse, (pos - half_size, pos + half_size));
                if hovered {
                    text = text.color(Rgb::new(0.5, 0.5, 0.5));

                    if ctx.input.mouse_pressed(MouseButton::Left) {
                        ctx.push_screen(GameScreen::load(world.clone()));
                    }
                }

                ctx.draw(text);
            }
        }
    }

    fn on_init(&mut self, state: &mut App) {
        // todo: make async with poll_promise?
        let world_dir = state.data_dir.join("sandbox");
        if world_dir.exists() {
            for world in world_dir.read_dir().unwrap().filter_map(Result::ok) {
                let path = world.path();
                let meta = match load_meta(&path) {
                    Ok(meta) => meta,
                    Err(err) => {
                        warn!("Failed to load meta for {:?}: {}", path, err);
                        continue;
                    }
                };

                self.worlds.push((path, meta));
            }
        }
    }
}

fn load_meta(path: &PathBuf) -> Result<BoardMeta> {
    let file = File::open(path)?;
    Ok(bincode::deserialize_from(file)?)
}
