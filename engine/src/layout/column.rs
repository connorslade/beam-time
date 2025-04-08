use nalgebra::Vector2;

use crate::graphics_context::GraphicsContext;

use super::{
    bounds::Bounds2D, container::Container, Direction, Justify, Layout, LayoutElement,
    SizedLayoutElement,
};

pub struct ColumnLayout {
    origin: Vector2<f32>,
    available: Vector2<f32>,
    container: Container,

    padding: f32,
    justify: Justify,
    direction: Direction,
}

impl ColumnLayout {
    pub fn new(padding: f32) -> Self {
        Self {
            origin: Vector2::zeros(),
            available: Vector2::repeat(f32::MAX),
            container: Container::default(),

            padding,
            justify: Justify::Min,
            direction: Direction::MinToMax,
        }
    }

    /// Set the starting available size of this layout element.
    pub fn sized(self, available: Vector2<f32>) -> Self {
        Self { available, ..self }
    }

    /// Set wether this element should be filled from top to bottom or bottom to
    /// top.
    pub fn direction(self, direction: Direction) -> Self {
        Self { direction, ..self }
    }

    /// If the column is wider than some of the elements added to it, how should
    /// they be justified. Either left, right, or center aligned.
    pub fn justify(self, justify: Justify) -> Self {
        Self { justify, ..self }
    }

    /// How much space is available to this container. If the root layout
    /// element was not given a defined size, this will return a very large
    /// number on the order of [`f32::MAX`].
    pub fn available(&self) -> Vector2<f32> {
        self.available
    }
}

impl Layout for ColumnLayout {
    fn layout(&mut self, ctx: &mut GraphicsContext, element: Box<dyn LayoutElement>) {
        let mut element = SizedLayoutElement::new(ctx, element);
        let size = element.bounds.size();

        match self.direction {
            Direction::MinToMax => {
                self.origin.y -= size.y;
                element.translate(self.origin);
                self.origin.y -= self.padding;
            }
            Direction::MaxToMin => {
                element.translate(self.origin);
                self.origin.y += size.y + self.padding;
            }
        }

        self.available.y -= size.y + self.padding;
        self.container.insert(element);
    }
}

impl LayoutElement for ColumnLayout {
    fn translate(&mut self, distance: Vector2<f32>) {
        self.container.translate(distance);
    }

    fn bounds(&self, _ctx: &mut GraphicsContext) -> Bounds2D {
        let offset = self
            .direction
            .is_flipped()
            .then(|| Vector2::y() * -self.container.bounds.height())
            .unwrap_or_default();
        self.container.bounds.translated(offset)
    }

    fn draw(mut self: Box<Self>, ctx: &mut GraphicsContext) {
        if self.direction.is_flipped() {
            let height = self.container.bounds.height();
            self.container.translate(Vector2::y() * -height);
        }

        let container_width = self.container.bounds.size().x;
        for child in &mut self.container.children {
            let width = child.bounds.size().x;
            let offset = self.justify.offset(container_width, width);
            child.translate(Vector2::x() * offset);
        }

        self.container.draw(ctx);
    }
}
