use nalgebra::Vector2;

use crate::graphics_context::Anchor;

pub struct Bounds2D {
    pub min: Vector2<f32>,
    pub max: Vector2<f32>,
}

pub trait LayoutElement {
    fn position(&mut self, position: Vector2<f32>, anchor: Anchor);
    fn bounds(&self) -> Bounds2D;
}

pub struct Container {
    bounds: Bounds2D,
    children: Vec<Box<dyn LayoutElement>>,
}
