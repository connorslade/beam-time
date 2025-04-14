use nalgebra::Vector2;
use tracker::{LayoutTracker, TrackedElement};

use crate::graphics_context::GraphicsContext;

use bounds::Bounds2D;
pub mod bounds;
pub mod column;
pub mod container;
pub mod root;
pub mod row;
pub mod tracker;

/// Elements that can be laid out programmatically.
pub trait LayoutElement {
    /// Shifts the element by the given distance.
    fn translate(&mut self, distance: Vector2<f32>);
    /// Gets the rectangular bounds of the element.
    fn bounds(&self, ctx: &mut GraphicsContext) -> Bounds2D;
    /// Draws the element.
    fn draw(self: Box<Self>, ctx: &mut GraphicsContext);

    /// Add the element to some other layout.
    fn layout(self, ctx: &mut GraphicsContext, layout: &mut dyn Layout)
    where
        Self: Sized + 'static,
    {
        layout.layout(ctx, Box::new(self));
    }
    /// Convert the element to a tracked element with the given layout tracker.
    fn tracked(self, tracker: LayoutTracker) -> TrackedElement<Self>
    where
        Self: Sized + 'static,
    {
        TrackedElement::new(tracker, self)
    }
}

/// System that can be passed layout elements and use the
/// [`LayoutElement::translate`] method to lay them out in some fashion.
pub trait Layout {
    /// Adds the specified element to the current layout.
    fn layout(&mut self, ctx: &mut GraphicsContext, element: Box<dyn LayoutElement>);
    /// How much space is available to this container. If the root layout
    /// element was not given a defined size, this will not return a positive
    /// number (≤ 0).
    fn available(&self) -> Vector2<f32>;
    /// Set the starting available size of this layout element.
    fn sized(&mut self, available: Vector2<f32>);
}

/// Convince methods for creating layouts.
pub trait LayoutMethods: Layout {
    fn nest<T: Layout + LayoutElement + 'static>(
        &mut self,
        ctx: &mut GraphicsContext,
        mut layout: T,
        ui: impl FnOnce(&mut GraphicsContext, &mut T),
    ) {
        let available = layout.available();
        if available.x == 0.0 && available.y == 0.0 {
            layout.sized(self.available());
        }

        ui(ctx, &mut layout);
        self.layout(ctx, Box::new(layout));
    }
}

pub struct SizedLayoutElement {
    pub element: Box<dyn LayoutElement>,
    pub bounds: Bounds2D,
}

#[derive(Debug, Clone, Copy)]
pub enum Justify {
    Min,
    Center,
    Max,
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    MinToMax,
    MaxToMin,
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

impl<T: Layout> LayoutMethods for T {}

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
