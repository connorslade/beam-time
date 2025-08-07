use engine::{
    drawable::text::Text,
    exports::nalgebra::Vector2,
    graphics_context::{Anchor, GraphicsContext},
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
