use std::f32::consts::TAU;

use itertools::Itertools;
use nalgebra::Vector2;

use crate::{
    color::Rgb,
    graphics_context::{Anchor, Drawable, GraphicsContext},
    layout::{LayoutElement, bounds::Bounds2D},
    render::shape::ShapeVertex,
};

#[derive(Clone)]
pub struct Circle {
    position: Vector2<f32>,
    anchor: Anchor,
    z_index: i16,
    color: Rgb<f32>,
    r: f32,
    n: u32,
}

impl Circle {
    pub fn new(r: f32) -> Self {
        Self {
            position: Vector2::zeros(),
            anchor: Anchor::BottomLeft,
            z_index: 0,
            color: Rgb::repeat(1.0),
            n: 32,
            r,
        }
    }

    pub fn position(self, position: Vector2<f32>, anchor: Anchor) -> Self {
        Self {
            position,
            anchor,
            ..self
        }
    }

    pub fn color(self, color: impl Into<Rgb<f32>>) -> Self {
        Self {
            color: color.into(),
            ..self
        }
    }

    pub fn z_index(self, z_index: i16) -> Self {
        Self { z_index, ..self }
    }

    pub fn points(self, n: u32) -> Self {
        Self { n, ..self }
    }
}

impl Drawable for Circle {
    fn draw(self, ctx: &mut GraphicsContext) {
        let r = self.r * ctx.scale_factor;
        let offset = self.anchor.offset(Vector2::repeat(r * 2.0));
        let center = self.position + Vector2::repeat(r) + offset;

        let center_vert = ctx.shapes.push_vertex(ShapeVertex {
            position: center,
            z_index: self.z_index,
            color: self.color,
        });

        (0..self.n)
            .map(|i| {
                let p = i as f32 / self.n as f32 * TAU;
                let position = Vector2::new(p.cos(), p.sin()) * r + center;
                ShapeVertex {
                    position,
                    z_index: self.z_index,
                    color: self.color,
                }
            })
            .cycle()
            .take(self.n as usize + 1)
            .tuple_windows()
            .for_each(|(a, b)| {
                let (a, b) = (ctx.shapes.push_vertex(a), ctx.shapes.push_vertex(b));
                ctx.shapes.push_triangle(&[center_vert, a, b]);
            });
    }
}

impl LayoutElement for Circle {
    fn translate(&mut self, distance: Vector2<f32>) {
        self.position += distance;
    }

    fn bounds(&self, ctx: &mut GraphicsContext) -> Bounds2D {
        let r = self.r * ctx.scale_factor;
        let size = Vector2::repeat(r * 2.0);

        let center = self.position + Vector2::repeat(r) + self.anchor.offset(size);
        Bounds2D::new(center, center + size)
    }

    fn draw(self: Box<Self>, ctx: &mut GraphicsContext) {
        (*self).draw(ctx);
    }
}
