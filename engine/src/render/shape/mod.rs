use nalgebra::Vector2;

use crate::color::Rgb;

pub mod pipeline;
mod render;

#[derive(Clone, Copy)]
pub struct ShapeVertex {
    pub position: Vector2<f32>,
    pub z_index: i16,
    pub color: Rgb<f32>,
}

#[derive(Default)]
pub struct GpuPolygons {
    pub(crate) vertices: Vec<ShapeVertex>,
    pub(crate) indices: Vec<u32>,
}

impl ShapeVertex {
    pub fn new(position: Vector2<f32>, color: impl Into<Rgb<f32>>) -> Self {
        Self {
            position,
            z_index: 0,
            color: color.into(),
        }
    }

    pub fn clip(mut self, [min, max]: [Vector2<f32>; 2]) -> Self {
        let clipped_x = self.position.x.clamp(min.x, max.x);
        let clipped_y = self.position.y.clamp(min.y, max.y);
        self.position = Vector2::new(clipped_x, clipped_y);
        self
    }

    pub fn z_index(mut self, z_index: i16) -> Self {
        self.z_index = z_index;
        self
    }
}

impl GpuPolygons {
    pub fn push_triangle(&mut self, vertices: &[ShapeVertex; 3]) {
        let start = self.vertices.len() as u32;
        self.vertices.extend_from_slice(vertices);
        self.indices
            .extend_from_slice(&[start, start + 1, start + 2]);
    }

    pub fn push_triangles(&mut self, triangles: &[[ShapeVertex; 3]]) {
        for triangle in triangles {
            self.push_triangle(triangle);
        }
    }

    pub fn push_quad(&mut self, vertices: &[ShapeVertex; 4]) {
        let start = self.vertices.len() as u32;
        self.vertices.extend_from_slice(vertices);
        self.indices
            .extend_from_slice(&[start, start + 1, start + 2]);
        self.indices
            .extend_from_slice(&[start + 2, start + 3, start]);
    }
}
