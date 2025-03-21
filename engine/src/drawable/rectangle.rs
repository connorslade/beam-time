use nalgebra::{Vector2, Vector3};

use crate::{
    color::Rgb,
    graphics_context::{Anchor, Drawable, GraphicsContext},
    render::shape::ShapeVertex,
};

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

        [
            offset + Vector2::new(0.0, 0.0),
            offset + Vector2::new(0.0, self.size.y),
            offset + self.size,
            offset + Vector2::new(self.size.x, 0.0),
        ]
    }
}

impl<App> Drawable<App> for Rectangle {
    fn draw(self, ctx: &mut GraphicsContext<App>) {
        // todo: move layer / div by window to render.rs
        let [a, b, c, d] = self.points();
        let z = (i16::MAX as f32 - self.z_index as f32) / (i16::MAX as f32 * 2.0);
        let window = ctx.size();

        let color = self.color.into();
        ctx.shapes.push_quad([
            ShapeVertex::new(a.component_div(&window).push(z), color),
            ShapeVertex::new(b.component_div(&window).push(z), color),
            ShapeVertex::new(c.component_div(&window).push(z), color),
            ShapeVertex::new(d.component_div(&window).push(z), color),
        ]);
    }
}
