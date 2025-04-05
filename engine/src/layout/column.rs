use nalgebra::Vector2;

use crate::graphics_context::GraphicsContext;

use super::{
    bounds::Bounds2D, container::Container, Justify, Layout, LayoutElement, SizedLayoutElement,
};

pub struct ColumnLayout {
    origin: Vector2<f32>,
    container: Container,

    padding: f32,
    justify: Justify,
}

impl ColumnLayout {
    pub fn new(padding: f32) -> Self {
        Self {
            origin: Vector2::zeros(),
            container: Container::default(),

            padding,
            justify: Justify::Min,
        }
    }

    pub fn justify(mut self, justify: Justify) -> Self {
        self.justify = justify;
        self
    }
}

impl Layout for ColumnLayout {
    fn layout(&mut self, ctx: &mut GraphicsContext, element: Box<dyn LayoutElement>) {
        let mut element = SizedLayoutElement::new(ctx, element);

        self.origin.y -= element.bounds.height();
        element.translate(self.origin);
        self.origin.y -= self.padding;

        self.container.insert(element);
    }
}

impl LayoutElement for ColumnLayout {
    fn translate(&mut self, distance: Vector2<f32>) {
        self.container.translate(distance);
    }

    fn bounds(&self, _ctx: &mut GraphicsContext) -> Bounds2D {
        self.container.bounds
    }

    fn draw(mut self: Box<Self>, ctx: &mut GraphicsContext) {
        let container_width = self.container.bounds.size().x;
        for child in &mut self.container.children {
            let width = child.bounds.size().x;
            let offset = self.justify.offset(container_width, width);
            child.translate(Vector2::x() * offset);
        }

        self.container.draw(ctx);
    }
}
