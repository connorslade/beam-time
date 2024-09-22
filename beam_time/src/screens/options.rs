use engine::{
    drawable::text::Text,
    exports::{
        nalgebra::Vector2,
        winit::keyboard::{KeyCode, PhysicalKey},
    },
    graphics_context::{Anchor, GraphicsContext},
    screens::Screen,
};

use crate::{
    assets::{ALAGARD_FONT, BACK_BUTTON},
    consts::{BACKGROUND_COLOR, FOREGROUND_COLOR},
    ui::{
        button::{Button, ButtonState},
        waterfall::Waterfall,
    },
    App,
};

#[derive(Default)]
pub struct OptionsScreen {
    back_button: ButtonState,
}

impl Screen<App> for OptionsScreen {
    fn render(&mut self, state: &mut App, ctx: &mut GraphicsContext<App>) {
        ctx.input
            .key_down(PhysicalKey::Code(KeyCode::Escape))
            .then(|| ctx.pop_screen());

        ctx.background(BACKGROUND_COLOR);
        ctx.draw(Waterfall::new(&mut state.waterfall));

        // Screen title
        let pos = Vector2::new(ctx.size().x / 2.0, ctx.size().y * 0.9);
        ctx.draw(
            Text::new(ALAGARD_FONT, "Options")
                .color(FOREGROUND_COLOR)
                .pos(pos, Anchor::TopCenter)
                .scale(Vector2::repeat(6.0)),
        );

        // Back button
        ctx.draw(
            Button::new(BACK_BUTTON, &mut self.back_button)
                .pos(Vector2::new(ctx.center().x, 10.0), Anchor::BottomCenter)
                .scale(Vector2::repeat(4.0))
                .on_click(|ctx| ctx.pop_screen()),
        );
    }

    fn on_resize(&mut self, state: &mut App, _size: Vector2<f32>) {
        state.waterfall.reset();
    }
}
