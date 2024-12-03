use std::mem;

use wgpu::{BufferAddress, VertexAttribute, VertexBufferLayout, VertexFormat, VertexStepMode};

use super::Vertex;

pub const VERTEX_BUFFER_LAYOUT: VertexBufferLayout = VertexBufferLayout {
    array_stride: mem::size_of::<Vertex>() as BufferAddress,
    step_mode: VertexStepMode::Vertex,
    attributes: &[
        VertexAttribute {
            format: VertexFormat::Float32x4,
            offset: 0,
            shader_location: 0,
        },
        VertexAttribute {
            format: VertexFormat::Float32x2,
            offset: 4 * 3,
            shader_location: 1,
        },
    ],
};
