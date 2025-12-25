use std::f32::consts::TAU;

use engine::{
    color::{OkLab, Rgb},
    drawable::Anchor,
    drawable::text::Text,
    exports::nalgebra::Vector2,
    graphics_context::GraphicsContext,
    render::sprite::GpuSprite,
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

pub fn rainbow_text(now: f32, sprites: &mut [GpuSprite]) {
    let count = sprites.len();
    for (idx, sprite) in sprites.iter_mut().enumerate() {
        let t = (idx / 2) as f32 / (count / 2) as f32;
        let color = OkLab::new(0.8, 0.1893, 0.0)
            .hue_shift(t * TAU - now * 2.0)
            .to_lrgb();
        sprite.color *= Rgb::new(color.r, color.g, color.b).map(|x| x as f32 / 255.0);

        let offset = (t * TAU - now * 6.0).sin() * 4.0;
        sprite.points.iter_mut().for_each(|point| point.y += offset);
    }
}
