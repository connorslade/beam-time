use nalgebra::Vector2;

use crate::graphics_context::GraphicsContext;

use super::{
    Direction, Justify, Layout, LayoutElement, SizedLayoutElement, bounds::Bounds2D,
    container::Container,
};

pub struct RowLayout {
    origin: Vector2<f32>,
    available: Vector2<f32>,
    container: Container,

    padding: f32,
    justify: Justify,
    direction: Direction,
}

impl RowLayout {
    pub fn new(padding: f32) -> Self {
        Self {
            origin: Vector2::zeros(),
            available: Vector2::repeat(0.0),
            container: Container::default(),

            padding,
            justify: Justify::Max,
            direction: Direction::MinToMax,
        }
    }

    /// Set the starting available size of this layout element.
    pub fn sized(self, available: Vector2<f32>) -> Self {
        Self { available, ..self }
    }

    /// Set wether this element should be filled from left to right or right to
    /// left.
    pub fn direction(self, direction: Direction) -> Self {
        Self { direction, ..self }
    }

    /// If the row is taller than some of the elements added to it, how should
    /// they be justified. Either top, bottom, or center aligned.
    pub fn justify(mut self, justify: Justify) -> Self {
        self.justify = justify;
        self
    }
}

impl Layout for RowLayout {
    fn layout(&mut self, ctx: &mut GraphicsContext, element: Box<dyn LayoutElement>) {
        let mut element = SizedLayoutElement::new(ctx, element);
        let width = element.bounds.width();

        match self.direction {
            Direction::MinToMax => {
                element.translate(self.origin);
                self.origin.x += width + self.padding;
            }
            Direction::MaxToMin => {
                self.origin.x -= width;
                element.translate(self.origin);
                self.origin.x -= self.padding;
            }
        }

        self.available.x -= width + self.padding;
        self.container.insert(element);
    }

    fn available(&self) -> Vector2<f32> {
        self.available
    }

    fn sized(&mut self, available: Vector2<f32>) {
        self.available = available;
    }
}

impl LayoutElement for RowLayout {
    fn translate(&mut self, distance: Vector2<f32>) {
        self.container.translate(distance);
    }

    fn bounds(&self, _ctx: &mut GraphicsContext) -> Bounds2D {
        let needs_offset = matches!(self.direction, Direction::MaxToMin);
        let offset = needs_offset
            .then(|| Vector2::x() * self.container.bounds.width())
            .unwrap_or_default();
        self.container.bounds.translated(offset)
    }

    fn draw(mut self: Box<Self>, ctx: &mut GraphicsContext) {
        if matches!(self.direction, Direction::MaxToMin) {
            let width = self.container.bounds.width();
            self.container.translate(Vector2::x() * width);
        }

        let container_width = self.container.bounds.size().y;
        for child in &mut self.container.children {
            let width = child.bounds.size().y;
            let offset = self.justify.offset(container_width, width);
            child.translate(Vector2::y() * offset);
        }

        self.container.draw(ctx);
    }
}

impl Clone for RowLayout {
    fn clone(&self) -> Self {
        Self {
            justify: self.justify,
            direction: self.direction,
            ..Self::new(self.padding)
        }
    }
}
