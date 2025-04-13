use nalgebra::Vector2;

use crate::{
    graphics_context::{Drawable, GraphicsContext},
    layout::{bounds::Bounds2D, LayoutElement},
};

pub struct DummyDrawable;

impl Drawable for DummyDrawable {
    fn draw(self, _ctx: &mut GraphicsContext) {}
}

impl LayoutElement for DummyDrawable {
    fn translate(&mut self, _distance: Vector2<f32>) {}

    fn bounds(&self, _ctx: &mut GraphicsContext) -> Bounds2D {
        Bounds2D::new(Vector2::repeat(0.0), Vector2::repeat(0.0))
    }

    fn draw(self: Box<Self>, _ctx: &mut GraphicsContext) {}
}
