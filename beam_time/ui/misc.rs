use engine::{
    drawable::Anchor, drawable::text::Text, exports::nalgebra::Vector2,
    graphics_context::GraphicsContext,
};

use crate::assets::UNDEAD_FONT;

pub fn tile_label(scale: f32, text_scale: f32, pos: Vector2<f32>, label: impl ToString) -> Text {
    let offset = Vector2::new(6.5, -7.5) * scale;
    Text::new(UNDEAD_FONT, label)
        .scale(Vector2::repeat(text_scale.max(0.75)))
        .position(pos + offset, Anchor::BottomRight)
}

pub fn body(max_width: f32) -> impl Fn(&str) -> Text {
    move |text| {
        Text::new(UNDEAD_FONT, text)
            .scale(Vector2::repeat(2.0))
            .max_width(max_width)
    }
}

#[inline(always)]
pub fn modal_size(ctx: &GraphicsContext) -> Vector2<f32> {
    Vector2::new((ctx.size().x * 0.75).clamp(400.0, 800.0), 250.0)
}

pub fn title_layout(ctx: &GraphicsContext, max_scale: f32) -> (f32, Vector2<f32>) {
    let size = ctx.size();

    let (x_scale, y_scale) = (size.x / 160.0, size.y / 70.0);
    let scale = (x_scale).min(y_scale).clamp(4.0, max_scale);

    let y_offset = (y_scale.min(max_scale) - 3.0) * 16.0;
    let pos = Vector2::new(size.x / 2.0, size.y - y_offset);

    (scale, pos)
}
