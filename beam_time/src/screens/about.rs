use engine::{
    drawable::text::Text,
    exports::{
        nalgebra::Vector2,
        winit::keyboard::{KeyCode, PhysicalKey},
    },
    graphics_context::{Anchor, GraphicsContext},
    screens::Screen,
};
use indoc::indoc;

use crate::{
    assets::{BACK_BUTTON, DEFAULT_FONT},
    consts::{BACKGROUND_COLOR, FOREGROUND_COLOR},
    ui::{
        button::{Button, ButtonState},
        waterfall::Waterfall,
    },
    App,
};

const DESCRIPTION: &str = indoc! {"
    Beam time is a logic puzzle about redirecting and splitting laser beams to create circuits. \
    It's made with a custom GPU accelerated game engine.
    
    Source code is available online at https://github.com/connorslade/beam-time.

    Assets Used:

    • Undead Pixel Light, Font by Not Jam
"};

#[derive(Default)]
pub struct AboutScreen {
    back_button: ButtonState,
}

impl Screen<App> for AboutScreen {
    fn render(&mut self, state: &mut App, ctx: &mut GraphicsContext<App>) {
        ctx.input
            .key_down(PhysicalKey::Code(KeyCode::Escape))
            .then(|| ctx.pop_screen());

        ctx.background(BACKGROUND_COLOR);
        ctx.draw(Waterfall::new(&mut state.waterfall));

        // Screen title
        let pos = Vector2::new(ctx.size().x / 2.0, ctx.size().y * 0.9);
        ctx.draw(
            Text::new(DEFAULT_FONT, "About")
                .color(FOREGROUND_COLOR)
                .pos(pos, Anchor::TopCenter)
                .scale(Vector2::repeat(6.0)),
        );

        ctx.draw(
            Text::new(DEFAULT_FONT, DESCRIPTION)
                .max_width(ctx.size().x - 20.0)
                .pos(Vector2::new(10.0, ctx.size().y * 0.8), Anchor::TopLeft)
                .color(FOREGROUND_COLOR)
                .scale(Vector2::repeat(3.0)),
        );

        // Back button
        ctx.draw(
            Button::new(BACK_BUTTON, &mut self.back_button)
                .pos(Vector2::new(10.0, 10.0), Anchor::BottomLeft)
                .scale(Vector2::repeat(2.0))
                .on_click(|ctx| ctx.pop_screen()),
        );
    }

    fn on_resize(&mut self, state: &mut App, _size: Vector2<f32>) {
        state.waterfall.reset();
    }
}
