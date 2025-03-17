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

impl GpuPolygons {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            indices: Vec::new(),
        }
    }
}
