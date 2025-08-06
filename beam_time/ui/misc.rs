use engine::{
    assets::FontRef,
    drawable::{sprite::Sprite, text::Text},
    exports::{nalgebra::Vector2, winit::keyboard::KeyCode},
    graphics_context::{Anchor, Drawable, GraphicsContext},
    memory::MemoryKey,
};

use crate::{
    app::App,
    assets::{ALAGARD_FONT, BACK_BUTTON, UNDEAD_FONT},
    consts::BACKGROUND_COLOR,
};

use super::components::button::Button;

pub fn titled_screen(
    state: &mut App,
    ctx: &mut GraphicsContext,
    back: Option<MemoryKey>,
    title: &str,
) -> Vector2<f32> {
    ctx.input
        .key_pressed(KeyCode::Escape)
        .then(|| state.pop_screen());

    ctx.background(BACKGROUND_COLOR);

    let pos = Vector2::new(ctx.size().x / 2.0, ctx.size().y * 0.9);
    Text::new(ALAGARD_FONT, title)
        .position(pos, Anchor::TopCenter)
        .scale(Vector2::repeat(6.0))
        .default_shadow()
        .draw(ctx);

    if let Some(back) = back {
        let back_pos = Vector2::new(ctx.center().x, 10.0 + 28.0 * ctx.scale_factor);
        let sprite = Sprite::new(BACK_BUTTON)
            .position(back_pos, Anchor::Center)
            .scale(Vector2::repeat(4.0));
        let button = Button::new(back, sprite);
        button.is_clicked(ctx).then(|| state.pop_screen());
        button.draw(ctx);
    }

    pos
}

pub fn font_scale(
    ctx: &mut GraphicsContext,
    font: FontRef,
    scale: f32,
    lines: usize,
) -> (f32, f32, f32) {
    let font_desc = &ctx.assets.get_font(font).desc;
    let line_height = font_desc.height * scale;
    let line_spacing = (line_height + font_desc.leading * scale) * ctx.scale_factor;
    let total_height = line_spacing * lines as f32;

    (line_height, line_spacing, total_height)
}

pub fn tile_label(
    ctx: &mut GraphicsContext,
    scale: f32,
    pos: Vector2<f32>,
    label: impl ToString,
) -> Text {
    let offset = scale * ctx.scale_factor;
    let offset = Vector2::new(6.5 * offset, -7.5 * offset);
    Text::new(UNDEAD_FONT, label)
        .scale(Vector2::repeat(scale / 2.0))
        .position(pos + offset, Anchor::BottomRight)
}

pub fn body(max_width: f32) -> impl Fn(&str) -> Text {
    move |text| {
        Text::new(UNDEAD_FONT, text)
            .scale(Vector2::repeat(2.0))
            .max_width(max_width)
    }
}
