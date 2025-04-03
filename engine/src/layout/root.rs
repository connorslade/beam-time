use nalgebra::Vector2;

use crate::graphics_context::{Anchor, GraphicsContext};

use super::{container::Container, LayoutElement};

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

    pub fn layout(&mut self, ctx: &mut GraphicsContext, mut element: impl LayoutElement + 'static) {
        let bounds = element.bounds(ctx).translated(self.origin);
        element.translate(self.origin);

        self.container.insert(bounds, element);
    }

    pub fn draw(mut self, ctx: &mut GraphicsContext) {
        let size = self.container.bounds(ctx).size();
        let shift = self.anchor.offset(size);
        self.container.translate(shift + Vector2::y() * size.y);

        self.container.draw(ctx);
    }
}
