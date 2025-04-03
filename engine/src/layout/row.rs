use nalgebra::Vector2;

use crate::graphics_context::GraphicsContext;

use super::{
    bounds::Bounds2D, container::Container, Justify, Layout, LayoutElement, SizedLayoutElement,
};

pub struct RowLayout {
    origin: Vector2<f32>,
    container: Container,

    padding: f32,
    justify: Justify,
}

impl RowLayout {
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

    // todo: Try to remove graphics context here?
    pub fn layout(&mut self, ctx: &mut GraphicsContext, element: impl LayoutElement + 'static) {
        let mut element = SizedLayoutElement::new(ctx, Box::new(element));

        element.translate(self.origin);
        self.origin.x += element.bounds.width() + self.padding;

        self.container.insert(element);
    }

    pub fn draw(mut self, ctx: &mut GraphicsContext) {
        let container_width = self.container.bounds.size().y;
        for child in &mut self.container.children {
            let width = child.bounds.size().y;
            let offset = self.justify.offset(container_width, width);
            child.translate(Vector2::y() * offset);
        }

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

    fn draw(self: Box<Self>, ctx: &mut GraphicsContext) {
        RowLayout::draw(*self, ctx);
    }
}

impl Layout for RowLayout {
    fn layout(&mut self, ctx: &mut GraphicsContext, element: impl LayoutElement + 'static) {
        self.layout(ctx, element);
    }
}
