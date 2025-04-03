use nalgebra::Vector2;

use crate::graphics_context::GraphicsContext;

use super::{bounds::Bounds2D, container::Container, LayoutElement};

pub struct RowLayout {
    origin: Vector2<f32>,
    container: Container,

    padding: f32,
}

impl RowLayout {
    pub fn new(padding: f32) -> Self {
        Self {
            origin: Vector2::zeros(),
            container: Container::default(),

            padding,
        }
    }

    // todo: Try to remove graphics context here?
    pub fn layout(&mut self, ctx: &mut GraphicsContext, mut element: impl LayoutElement + 'static) {
        let mut bounds = element.bounds(ctx);

        element.translate(self.origin);
        bounds.translate(self.origin);
        self.origin.x += bounds.width() + self.padding;

        self.container.insert(bounds, element);
    }

    pub fn draw(self, ctx: &mut GraphicsContext) {
        self.container.draw(ctx);
    }
}

impl LayoutElement for RowLayout {
    fn translate(&mut self, distance: Vector2<f32>) {
        self.container.translate(distance);
    }

    fn bounds(&self, ctx: &mut GraphicsContext) -> Bounds2D {
        self.container.bounds(ctx)
    }

    fn draw(&self, ctx: &mut GraphicsContext) {
        self.container.draw(ctx);
    }
}
