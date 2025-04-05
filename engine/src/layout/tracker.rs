use nalgebra::Vector2;

use crate::{graphics_context::GraphicsContext, memory::MemoryKey, memory_key};

use super::{bounds::Bounds2D, LayoutElement};

#[derive(Clone, Copy)]
pub struct LayoutTracker {
    key: MemoryKey,
}

impl LayoutTracker {
    pub fn new(key: MemoryKey) -> Self {
        Self {
            key: key.context(memory_key!()),
        }
    }

    pub fn bounds(&self, ctx: &mut GraphicsContext) -> Option<Bounds2D> {
        ctx.memory.get::<Bounds2D>(self.key).copied()
    }

    pub fn hovered(&self, ctx: &mut GraphicsContext) -> bool {
        self.bounds(ctx)
            .map(|x| x.contains(ctx.input.mouse))
            .unwrap_or_default()
    }
}

pub struct TrackedElement<T: LayoutElement> {
    element: T,
    key: MemoryKey,
}

impl<T: LayoutElement + 'static> TrackedElement<T> {
    pub fn new(tracker: LayoutTracker, element: T) -> Self {
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
