use std::mem;

use nalgebra::Vector2;
use wgpu::{
    AddressMode, BufferAddress, FilterMode, SamplerDescriptor, VertexAttribute, VertexBufferLayout,
    VertexFormat, VertexStepMode,
};

use crate::{assets::TextureRef, color::Rgb};

pub mod pipeline;
pub mod render;
use render::Instance;

#[derive(Debug)]
pub struct GpuSprite {
    pub texture: TextureRef,

    pub points: [Vector2<f32>; 4],
    pub uv: [Vector2<f32>; 2],
    pub clip: [Vector2<f32>; 2],

    pub z_index: i16,
    pub color: Rgb<f32>,
}

pub const INITIAL_BUFFER_SIZE: u64 = 2 << 11;

pub const SPRITE_INSTANCE_BUFFER_LAYOUT: VertexBufferLayout = VertexBufferLayout {
    array_stride: mem::size_of::<Instance>() as BufferAddress,
    step_mode: VertexStepMode::Instance,
    attributes: &[
        VertexAttribute {
            format: VertexFormat::Float32x4,
            offset: 0,
            shader_location: 1,
        },
        VertexAttribute {
            format: VertexFormat::Float32x4,
            offset: 4 * 4,
            shader_location: 2,
        },
        VertexAttribute {
            format: VertexFormat::Uint32,
            offset: 4 * 8,
            shader_location: 3,
        },
        VertexAttribute {
            format: VertexFormat::Float32x4,
            offset: 4 * 9,
            shader_location: 4,
        },
        VertexAttribute {
            format: VertexFormat::Float32,
            offset: 4 * 13,
            shader_location: 5,
        },
        VertexAttribute {
            format: VertexFormat::Float32x3,
            offset: 4 * 14,
            shader_location: 6,
        },
        VertexAttribute {
            format: VertexFormat::Float32x4,
            offset: 4 * 17,
            shader_location: 7,
        },
    ],
};

pub const SAMPLER_DESCRIPTOR: SamplerDescriptor = SamplerDescriptor {
    label: None,
    address_mode_u: AddressMode::ClampToEdge,
    address_mode_v: AddressMode::ClampToEdge,
    address_mode_w: AddressMode::ClampToEdge,
    mag_filter: FilterMode::Nearest,
    min_filter: FilterMode::Nearest,
    mipmap_filter: FilterMode::Nearest,
    lod_min_clamp: 0.0,
    lod_max_clamp: 0.0,
    compare: None,
    anisotropy_clamp: 1,
    border_color: None,
};
