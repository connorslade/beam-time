use engine::{
    color::Rgb,
    drawable::{shape::rectangle_outline::RectangleOutline, sprite::Sprite},
    exports::nalgebra::Vector2,
    graphics_context::{Anchor, Drawable, GraphicsContext},
};

use crate::{app::App, assets::TILE_EMITTER_RIGHT};

use super::Screen;

#[derive(Default)]
pub struct LayoutTestScreen {}

impl Screen for LayoutTestScreen {
    fn render(&mut self, state: &mut App, ctx: &mut GraphicsContext) {
        let t = state.start.elapsed().as_secs_f32() * 2.0;
        let scale = Vector2::repeat(4.0 + (t.sin() / 2.0) - 0.5);
        let pos = ctx.input.mouse;

        Sprite::new(TILE_EMITTER_RIGHT)
            .uv_offset(Vector2::x() * 16)
            .scale(Vector2::repeat(4.0))
            .dynamic_scale(scale, Anchor::Center)
            .position(pos, Anchor::BottomLeft)
            .draw(ctx);
        RectangleOutline::new(Vector2::repeat(4.0) * 16.0 * ctx.scale_factor, 1.0)
            .position(pos, Anchor::BottomLeft)
            .color(Rgb::hex(0x69bdd2))
            .draw(ctx);
    }
}
