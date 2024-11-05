use engine::{
    assets::FontRef,
    drawable::text::Text,
    exports::{nalgebra::Vector2, winit::keyboard::KeyCode},
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
        .key_pressed(KeyCode::Escape)
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

pub fn font_scale<App>(
    ctx: &mut GraphicsContext<App>,
    font: FontRef,
    scale: f32,
    lines: usize,
) -> (f32, f32, f32) {
    let font_desc = &ctx.assets.get_font(font).desc;
    let line_height = font_desc.height * scale;
    let line_spacing = (line_height + 2.0 * font_desc.leading * scale * ctx.scale_factor) * 1.5;
    let total_height = line_spacing * lines as f32;

    (line_height, line_spacing, total_height)
}
