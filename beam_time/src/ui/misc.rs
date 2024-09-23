use engine::{
    drawable::text::Text,
    exports::{
        nalgebra::Vector2,
        winit::keyboard::{KeyCode, PhysicalKey},
    },
    graphics_context::{Anchor, GraphicsContext},
};

use crate::{
    app::App,
    assets::{ALAGARD_FONT, BACK_BUTTON},
    consts::{BACKGROUND_COLOR, FOREGROUND_COLOR},
};

use super::{
    button::{Button, ButtonState},
    waterfall::Waterfall,
};

pub fn titled_screen(
    state: &mut App,
    ctx: &mut GraphicsContext<App>,
    back: &mut ButtonState,
    title: &str,
) -> Vector2<f32> {
    ctx.input.resized.then(|| state.waterfall.reset());
    ctx.input
        .key_down(PhysicalKey::Code(KeyCode::Escape))
        .then(|| ctx.pop_screen());

    ctx.background(BACKGROUND_COLOR);
    ctx.draw(Waterfall::new(&mut state.waterfall));

    let pos = Vector2::new(ctx.size().x / 2.0, ctx.size().y * 0.9);
    ctx.draw(
        Text::new(ALAGARD_FONT, title)
            .color(FOREGROUND_COLOR)
            .pos(pos, Anchor::TopCenter)
            .scale(Vector2::repeat(6.0)),
    );

    ctx.draw(
        Button::new(BACK_BUTTON, back)
            .pos(Vector2::new(ctx.center().x, 10.0), Anchor::BottomCenter)
            .scale(Vector2::repeat(4.0))
            .set_back()
            .on_click(|ctx| ctx.pop_screen()),
    );

    pos
}
