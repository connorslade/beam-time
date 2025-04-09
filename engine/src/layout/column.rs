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
            available: Vector2::repeat(0.0),
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
}

impl Layout for ColumnLayout {
    fn layout(&mut self, ctx: &mut GraphicsContext, element: Box<dyn LayoutElement>) {
        let mut element = SizedLayoutElement::new(ctx, element);
        let height = element.bounds.height();

        match self.direction {
            Direction::MinToMax => {
                self.origin.y -= height;
                element.translate(self.origin);
                self.origin.y -= self.padding;
            }
            Direction::MaxToMin => {
                element.translate(self.origin);
                self.origin.y += height + self.padding;
            }
        }

        self.available.y -= height + self.padding;
        self.container.insert(element);
    }

    fn available(&self) -> Vector2<f32> {
        self.available
    }

    fn sized(&mut self, available: Vector2<f32>) {
        self.available = available;
    }
}

impl LayoutElement for ColumnLayout {
    fn translate(&mut self, distance: Vector2<f32>) {
        self.container.translate(distance);
    }

    fn bounds(&self, _ctx: &mut GraphicsContext) -> Bounds2D {
        let needs_offset = matches!(self.direction, Direction::MinToMax);
        let offset = needs_offset
            .then(|| Vector2::y() * self.container.bounds.height())
            .unwrap_or_default();
        self.container.bounds.translated(offset)
    }

    fn draw(mut self: Box<Self>, ctx: &mut GraphicsContext) {
        // When laying items out going down, the first item's top left point is
        // put at the origin. Any items after that are put beneath it. Here we
        // shift the whole container up so the bottom left corner of the last
        // element is at the origin.
        if matches!(self.direction, Direction::MinToMax) {
            let height = self.container.bounds.height();
            self.container.translate(Vector2::y() * height);
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
