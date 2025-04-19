use std::f32::consts::TAU;

use itertools::Itertools;
use nalgebra::Vector2;

use crate::{
    color::Rgb,
    graphics_context::{Anchor, Drawable, GraphicsContext},
    render::shape::ShapeVertex,
};

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

        let center_vert = ShapeVertex {
            position: center,
            z_index: self.z_index,
            color: self.color,
        };

        (0..self.n)
            .into_iter()
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
                ctx.shapes.push_triangle(&[center_vert, a, b]);
            });
    }
}
