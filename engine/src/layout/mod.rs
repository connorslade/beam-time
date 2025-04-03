use nalgebra::Vector2;

use crate::graphics_context::GraphicsContext;

use bounds::Bounds2D;
pub mod bounds;
pub mod column;
pub mod container;
pub mod root;

pub trait LayoutElement {
    /// Shifts the element by the given distance.
    fn translate(&mut self, distance: Vector2<f32>);
    /// Gets the rectangular bounds of the element.
    fn bounds(&self, ctx: &mut GraphicsContext) -> Bounds2D;
    /// Draws the element.
    fn draw(&self, ctx: &mut GraphicsContext);
}
