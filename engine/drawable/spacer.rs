use nalgebra::Vector2;

use crate::{
    graphics_context::GraphicsContext,
    layout::{bounds::Bounds2D, LayoutElement},
};

#[derive(Clone)]
pub struct Spacer {
    bounds: Bounds2D,
}

impl Spacer {
    pub fn new(size: Vector2<f32>) -> Self {
        Self {
            bounds: Bounds2D::new(Vector2::zeros(), size),
        }
    }

    pub fn new_x(size: f32) -> Self {
        Self::new(Vector2::x() * size)
    }

    pub fn new_y(size: f32) -> Self {
        Self::new(Vector2::y() * size)
    }
}

impl LayoutElement for Spacer {
    fn translate(&mut self, distance: Vector2<f32>) {
        self.bounds.translate(distance);
    }

    fn bounds(&self, _ctx: &mut GraphicsContext) -> Bounds2D {
        self.bounds
    }

    fn draw(self: Box<Self>, _ctx: &mut GraphicsContext) {}
}
