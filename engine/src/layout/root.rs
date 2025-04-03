use nalgebra::Vector2;

use crate::graphics_context::GraphicsContext;

use super::{container::Container, LayoutElement};

pub struct RootLayout {
    container: Container,

    origin: Vector2<f32>,
}

impl RootLayout {
    pub fn new(origin: Vector2<f32>) -> Self {
        Self {
            container: Container::default(),
            origin,
        }
    }

    pub fn layout(&mut self, ctx: &mut GraphicsContext, mut element: impl LayoutElement + 'static) {
        let bounds = element.bounds(ctx).translated(self.origin);
        element.translate(self.origin);

        self.container.insert(bounds, element);
    }

    pub fn draw(self, ctx: &mut GraphicsContext) {
        self.container.draw(ctx);
    }
}
