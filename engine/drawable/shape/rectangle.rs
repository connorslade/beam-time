use nalgebra::Vector2;

use crate::{
    color::Rgb,
    drawable::RECTANGLE_POINTS,
    graphics_context::{Anchor, Drawable, GraphicsContext},
    layout::{LayoutElement, bounds::Bounds2D},
    render::shape::ShapeVertex,
};

#[derive(Clone)]
pub struct Rectangle {
    position: Vector2<f32>,
    position_anchor: Anchor,
    z_index: i16,
    size: Vector2<f32>,
    color: Rgb<f32>,
}

impl Rectangle {
    pub fn new(size: Vector2<f32>) -> Self {
        Self {
            position: Vector2::zeros(),
            position_anchor: Anchor::BottomLeft,
            z_index: 0,
            size,
            color: Rgb::repeat(1.0),
        }
    }

    pub fn position(mut self, position: Vector2<f32>, anchor: Anchor) -> Self {
        self.position = position;
        self.position_anchor = anchor;
        self
    }

    pub fn color(mut self, color: impl Into<Rgb<f32>>) -> Self {
        self.color = color.into();
        self
    }

    pub fn z_index(mut self, z_index: i16) -> Self {
        self.z_index = z_index;
        self
    }

    fn points(&self) -> [Vector2<f32>; 4] {
        let offset = self.position + self.position_anchor.offset(self.size);
        RECTANGLE_POINTS.map(|x| offset + x.component_mul(&self.size))
    }
}

impl Drawable for Rectangle {
    fn draw(self, ctx: &mut GraphicsContext) {
        let verts = self.points().map(|x| {
            ctx.shapes
                .push_vertex(ShapeVertex::new(x, self.color).z_index(self.z_index))
        });
        ctx.shapes.push_quad(verts);
    }
}

impl LayoutElement for Rectangle {
    fn translate(&mut self, distance: Vector2<f32>) {
        self.position += distance;
    }

    fn bounds(&self, _ctx: &mut GraphicsContext) -> Bounds2D {
        let pos = self.position + self.position_anchor.offset(self.size);
        Bounds2D::new(pos, pos + self.size)
    }

    fn draw(self: Box<Self>, ctx: &mut GraphicsContext) {
        (*self).draw(ctx);
    }
}
