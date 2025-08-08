use nalgebra::Vector2;

use crate::{
    drawable::Drawable,
    graphics_context::GraphicsContext,
    layout::{LayoutElement, bounds::Bounds2D},
};

#[derive(Default)]
pub struct DummyDrawable {
    position: Vector2<f32>,
}

impl DummyDrawable {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Drawable for DummyDrawable {
    fn draw(self, _ctx: &mut GraphicsContext) {}
}

impl LayoutElement for DummyDrawable {
    fn translate(&mut self, distance: Vector2<f32>) {
        self.position += distance;
    }

    fn bounds(&self, _ctx: &mut GraphicsContext) -> Bounds2D {
        Bounds2D::new(self.position, self.position)
    }

    fn draw(self: Box<Self>, _ctx: &mut GraphicsContext) {}
}
