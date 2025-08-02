use nalgebra::Vector2;

use crate::{
    graphics_context::GraphicsContext,
    layout::{LayoutElement, bounds::Bounds2D},
};

pub struct NoPadding<T: LayoutElement> {
    inner: T,
}

pub trait NoPaddingExt<T: LayoutElement> {
    fn no_padding(self) -> NoPadding<T>;
}

impl<T: LayoutElement> LayoutElement for NoPadding<T> {
    fn translate(&mut self, distance: Vector2<f32>) {
        self.inner.translate(distance);
    }

    fn bounds(&self, ctx: &mut GraphicsContext) -> Bounds2D {
        self.inner.bounds(ctx)
    }

    fn draw(self: Box<Self>, ctx: &mut GraphicsContext) {
        Box::new(self.inner).draw(ctx);
    }

    fn wants_padding(&self) -> bool {
        false
    }
}

impl<T: LayoutElement> NoPaddingExt<T> for T {
    fn no_padding(self) -> NoPadding<T> {
        NoPadding { inner: self }
    }
}
