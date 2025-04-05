use nalgebra::Vector2;

use crate::graphics_context::{Anchor, Drawable, GraphicsContext};

use super::{container::Container, Layout, LayoutElement, SizedLayoutElement};

pub struct RootLayout {
    container: Container,

    origin: Vector2<f32>,
    anchor: Anchor,
}

impl RootLayout {
    pub fn new(origin: Vector2<f32>, anchor: Anchor) -> Self {
        Self {
            container: Container::default(),
            origin,
            anchor,
        }
    }
}

impl Layout for RootLayout {
    fn layout(&mut self, ctx: &mut GraphicsContext, element: Box<dyn LayoutElement>) {
        let mut element = SizedLayoutElement::new(ctx, element);
        element.translate(self.origin);

        self.container.insert(element);
    }
}

impl Drawable for RootLayout {
    fn draw(mut self, ctx: &mut GraphicsContext) {
        let size = self.container.bounds.size();
        let shift = self.anchor.offset(size);
        self.container.translate(shift + Vector2::y() * size.y);

        self.container.draw(ctx);
    }
}
