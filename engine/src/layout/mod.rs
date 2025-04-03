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

pub trait Layout {
    fn layout(&mut self, ctx: &mut GraphicsContext, element: impl LayoutElement + 'static);
}

pub struct SizedLayoutElement {
    pub element: Box<dyn LayoutElement>,
    pub bounds: Bounds2D,
}

#[derive(Debug)]
pub enum Justify {
    Min,
    Center,
    Max,
}

impl SizedLayoutElement {
    pub fn new(ctx: &mut GraphicsContext, element: Box<dyn LayoutElement>) -> Self {
        let bounds = element.bounds(ctx);
        Self { element, bounds }
    }
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

impl LayoutElement for SizedLayoutElement {
    fn translate(&mut self, distance: Vector2<f32>) {
        self.bounds.translate(distance);
        self.element.translate(distance);
    }

    fn bounds(&self, _ctx: &mut GraphicsContext) -> Bounds2D {
        self.bounds
    }

    fn draw(self: Box<Self>, ctx: &mut GraphicsContext) {
        self.element.draw(ctx);
    }
}
