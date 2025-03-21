use bytemuck::{Pod, Zeroable};
use nalgebra::Vector3;

pub mod pipeline;
mod render;

#[derive(Pod, Zeroable, Clone, Copy)]
#[repr(C)]
pub struct ShapeVertex {
    position: Vector3<f32>,
    color: Vector3<f32>,
}

pub struct GpuPolygons {
    vertices: Vec<ShapeVertex>,
    indices: Vec<u16>,
}

impl ShapeVertex {
    pub fn new(position: Vector3<f32>, color: Vector3<f32>) -> Self {
        Self { position, color }
    }
}

impl GpuPolygons {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            indices: Vec::new(),
        }
    }

    pub fn push_triangle(&mut self, vertices: [ShapeVertex; 3]) {
        let start = self.vertices.len() as u16;
        self.vertices.extend_from_slice(&vertices);
        self.indices
            .extend_from_slice(&[start, start + 1, start + 2]);
    }

    pub fn push_quad(&mut self, vertices: [ShapeVertex; 4]) {
        let start = self.vertices.len() as u16;
        self.vertices.extend_from_slice(&vertices);
        self.indices
            .extend_from_slice(&[start, start + 1, start + 2]);
        self.indices
            .extend_from_slice(&[start + 2, start + 3, start]);
    }
}
