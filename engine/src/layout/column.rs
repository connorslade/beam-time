use nalgebra::Vector2;

use crate::graphics_context::GraphicsContext;

use super::{bounds::Bounds2D, container::Container, LayoutElement};

pub struct ColumnLayout {
    origin: Vector2<f32>,
    container: Container,

    padding: f32,
}

impl ColumnLayout {
    pub fn new(padding: f32) -> Self {
        Self {
            origin: Vector2::zeros(),
            container: Container::default(),

            padding,
        }
    }

    // todo: Try to remove graphics context here?
    pub fn layout(&mut self, ctx: &mut GraphicsContext, mut element: impl LayoutElement + 'static) {
        let bounds = element.bounds(ctx);

        self.origin.y -= bounds.height() - self.padding;
        element.translate(self.origin);

        self.container.insert(bounds, element);
    }

    pub fn draw(self, ctx: &mut GraphicsContext) {
        self.container.draw(ctx);
    }
}

impl LayoutElement for ColumnLayout {
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
