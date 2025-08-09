use engine::{
    drawable::Anchor, drawable::text::Text, exports::nalgebra::Vector2,
    graphics_context::GraphicsContext,
};

use crate::assets::UNDEAD_FONT;

pub fn tile_label(
    ctx: &mut GraphicsContext,
    scale: f32,
    pos: Vector2<f32>,
    label: impl ToString,
) -> Text {
    let offset = scale * ctx.scale_factor;
    let offset = Vector2::new(6.5 * offset, -7.5 * offset);
    Text::new(UNDEAD_FONT, label)
        .scale(Vector2::repeat((scale / 2.0).max(ctx.scale_factor * 0.75)))
        .position(pos + offset, Anchor::BottomRight)
}

pub fn body(max_width: f32) -> impl Fn(&str) -> Text {
    move |text| {
        Text::new(UNDEAD_FONT, text)
            .scale(Vector2::repeat(2.0))
            .max_width(max_width)
    }
}

/// => (Margin, Padding)
#[inline(always)]
pub fn spacing(ctx: &GraphicsContext) -> (f32, f32) {
    let margin = 16.0 * ctx.scale_factor;
    let padding = 10.0 * ctx.scale_factor;
    (margin, padding)
}

#[inline(always)]
pub fn modal_size(ctx: &GraphicsContext) -> Vector2<f32> {
    let min_width = 400.0 * ctx.scale_factor;
    let max_width = 800.0 * ctx.scale_factor;
    Vector2::new(
        (ctx.size().x * 0.75).clamp(min_width, max_width),
        250.0 * ctx.scale_factor,
    )
}

pub fn title_layout(ctx: &GraphicsContext, max_scale: f32) -> (f32, Vector2<f32>) {
    let size = ctx.size();
    let screen = size / ctx.scale_factor;

    let (x_scale, y_scale) = (screen.x / 160.0, screen.y / 70.0);
    let scale = (x_scale).min(y_scale).clamp(4.0, max_scale);

    let y_offset = (y_scale.min(max_scale) - 3.0) * 16.0 * ctx.scale_factor;
    let pos = Vector2::new(size.x / 2.0, size.y - y_offset);

    (scale, pos)
}
