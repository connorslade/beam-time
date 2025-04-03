use nalgebra::Vector2;

use crate::graphics_context::GraphicsContext;

use bounds::Bounds2D;
pub mod bounds;
pub mod column;
pub mod container;
pub mod root;
pub mod row;

pub trait LayoutElement {
    /// Shifts the element by the given distance.
    fn translate(&mut self, distance: Vector2<f32>);
    /// Gets the rectangular bounds of the element.
    fn bounds(&self, ctx: &mut GraphicsContext) -> Bounds2D;
    /// Draws the element.
    fn draw(self: Box<Self>, ctx: &mut GraphicsContext);
}

#[derive(Debug)]
pub enum Justify {
    Min,
    Center,
    Max,
}

impl Justify {
    pub fn offset(&self, container: f32, element: f32) -> f32 {
        match self {
            Justify::Min => 0.0,
            Justify::Center => (container - element) / 2.0,
            Justify::Max => container - element,
        }
    }
}
