use std::path::PathBuf;

use engine::{
    color::Rgb,
    drawable::text::Text,
    exports::{nalgebra::Vector2, winit::event::MouseButton},
    graphics_context::{Anchor, GraphicsContext},
    screens::Screen,
};

use crate::{
    assets::UNDEAD_FONT,
    screens::game::GameScreen,
    ui::{button::ButtonState, misc::titled_screen},
    App,
};

#[derive(Default)]
pub struct SandboxScreen {
    worlds: Vec<PathBuf>,
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

            let font_desc = &ctx.assets.get_font(UNDEAD_FONT).desc;
            let line_height = font_desc.height * SCALE;
            let line_spacing = line_height + (font_desc.leading * 2.0) * SCALE;
            let total_height = line_spacing * self.worlds.len() as f32;

            for (i, world) in self.worlds.iter().enumerate() {
                let name = world.file_name().unwrap().to_str().unwrap();
                let pos =
                    ctx.center() + Vector2::new(0.0, total_height / 2.0 - line_spacing * i as f32);

                let mut text = Text::new(UNDEAD_FONT, name)
                    .pos(pos, Anchor::Center)
                    .scale(Vector2::repeat(SCALE));

                let width = text.width(ctx) * SCALE;
                let half_size = Vector2::new(width / 2.0, line_height / 2.0);
                let hovered = in_bounds(ctx.input.mouse, (pos - half_size, pos + half_size));
                if hovered {
                    text = text.color(Rgb::new(0.5, 0.5, 0.5));

                    if ctx.input.mouse_pressed(MouseButton::Left) {
                        ctx.pop_screen();
                        ctx.push_screen(GameScreen::new(world.clone()));
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
            self.worlds = world_dir
                .read_dir()
                .unwrap()
                .map(|e| e.unwrap().path())
                .collect();
        }
    }
}

fn in_bounds(pos: Vector2<f32>, bounds: (Vector2<f32>, Vector2<f32>)) -> bool {
    pos.x >= bounds.0.x && pos.x <= bounds.1.x && pos.y >= bounds.0.y && pos.y <= bounds.1.y
}
