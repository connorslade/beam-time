use nalgebra::Vector2;

use crate::{graphics_context::GraphicsContext, memory::MemoryKey};

use super::{bounds::Bounds2D, LayoutElement};

#[derive(Clone, Copy)]
pub struct HoverTracker {
    key: MemoryKey,
}

impl HoverTracker {
    pub fn new(key: MemoryKey) -> Self {
        Self { key }
    }

    pub fn hovered(&self, ctx: &mut GraphicsContext) -> bool {
        // todo: use bounds wrapper to avoid memory key collisions?
        let bounds = match ctx.memory.get::<Bounds2D>(self.key) {
            Some(bounds) => bounds,
            None => return false,
        };

        bounds.contains(ctx.input.mouse)
    }
}

pub struct TrackedElement<T: LayoutElement> {
    element: T,
    key: MemoryKey,
}

impl<T: LayoutElement> TrackedElement<T> {
    pub fn new(tracker: HoverTracker, element: T) -> Self {
        Self {
            key: tracker.key,
            element,
        }
    }

    pub fn into_inner(self) -> T {
        self.element
    }
}

impl<T: LayoutElement> LayoutElement for TrackedElement<T> {
    fn translate(&mut self, distance: Vector2<f32>) {
        self.element.translate(distance);
    }

    fn bounds(&self, ctx: &mut GraphicsContext) -> Bounds2D {
        self.element.bounds(ctx)
    }

    fn draw(self: Box<Self>, ctx: &mut GraphicsContext) {
        let bounds = self.element.bounds(ctx);
        ctx.memory.insert(self.key, bounds);

        Box::new(self.element).draw(ctx)
    }
}
