use nalgebra::Vector2;

use crate::{
    color::Rgb,
    drawable::{Anchor, Drawable},
    graphics_context::GraphicsContext,
    render::shape::ShapeVertex,
};

use super::circle::Circle;

#[derive(Clone)]
pub struct Line {
    points: [Vector2<f32>; 2],
    thickness: f32,
    color: Rgb<f32>,
    cap: LineCap,
    z_index: i16,
}

#[derive(Clone)]
pub enum LineCap {
    Butt, // why is it called this
    Round,
}

impl Line {
    pub fn new(a: Vector2<f32>, b: Vector2<f32>) -> Self {
        Self {
            points: [a, b],
            thickness: 4.0,
            color: Rgb::repeat(1.0),
            cap: LineCap::Butt,
            z_index: 0,
        }
    }

    pub fn points(self, a: Vector2<f32>, b: Vector2<f32>) -> Self {
        Self {
            points: [a, b],
            ..self
        }
    }

    pub fn thickness(self, thickness: f32) -> Self {
        Self { thickness, ..self }
    }

    pub fn color(self, color: Rgb<f32>) -> Self {
        Self { color, ..self }
    }

    pub fn cap(self, cap: LineCap) -> Self {
        Self { cap, ..self }
    }

    pub fn z_index(self, z_index: i16) -> Self {
        Self { z_index, ..self }
    }
}

impl Drawable for Line {
    fn draw(self, ctx: &mut GraphicsContext) {
        let [a, b] = self.points;
        let direction = (b - a).normalize();
        let p = Vector2::new(-direction.y, direction.x) * (self.thickness * ctx.scale_factor / 2.0);

        let mut vert = |position| {
            ctx.shapes.push_vertex(ShapeVertex {
                position,
                z_index: self.z_index,
                color: self.color,
            })
        };

        let points = [vert(a + p), vert(a - p), vert(b - p), vert(b + p)];
        ctx.shapes.push_quad(points);

        match self.cap {
            LineCap::Butt => {}
            LineCap::Round => {
                for point in [a, b] {
                    Circle::new(self.thickness / 2.0)
                        .position(point, Anchor::Center)
                        .color(self.color)
                        .z_index(self.z_index)
                        .draw(ctx);
                }
            }
        }
    }
}
