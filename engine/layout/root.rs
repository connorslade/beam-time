use nalgebra::Vector2;

use crate::{
    drawable::{Anchor, Drawable},
    graphics_context::GraphicsContext,
};

use super::{Layout, LayoutElement, SizedLayoutElement, container::Container};

/// This is where all layout hierarchies start.
pub struct RootLayout {
    available: Vector2<f32>,
    container: Container,

    origin: Vector2<f32>,
    anchor: Anchor,
}

impl RootLayout {
    pub fn new(origin: Vector2<f32>, anchor: Anchor) -> Self {
        Self {
            available: Vector2::zeros(),
            container: Container::default(),
            origin,
            anchor,
        }
    }

    /// Set the starting available size of this layout element.
    pub fn sized(self, available: Vector2<f32>) -> Self {
        Self { available, ..self }
    }
}

impl Layout for RootLayout {
    fn layout(&mut self, ctx: &mut GraphicsContext, element: Box<dyn LayoutElement>) {
        let mut element = SizedLayoutElement::new(ctx, element);
        element.translate(self.origin);

        self.container.insert(element);
    }

    fn available(&self) -> Vector2<f32> {
        self.available
    }

    fn sized(&mut self, available: Vector2<f32>) {
        self.available = available;
    }
}

impl Drawable for RootLayout {
    fn draw(mut self, ctx: &mut GraphicsContext) {
        let size = self.container.bounds.size();
        let shift = self.anchor.offset(size);
        self.container.translate(shift);

        self.container.draw(ctx);
    }
}
