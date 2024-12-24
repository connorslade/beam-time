use std::mem;

use wgpu::{
    AddressMode, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType, BufferAddress,
    FilterMode, SamplerBindingType, SamplerDescriptor, ShaderStages, TextureSampleType,
    TextureViewDimension, VertexAttribute, VertexBufferLayout, VertexFormat, VertexStepMode,
};

use super::render::Instance;

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
            format: VertexFormat::Float32x4,
            offset: 4 * 8,
            shader_location: 3,
        },
        VertexAttribute {
            format: VertexFormat::Float32,
            offset: 4 * 12,
            shader_location: 4,
        },
        VertexAttribute {
            format: VertexFormat::Float32x3,
            offset: 4 * 13,
            shader_location: 5,
        },
        VertexAttribute {
            format: VertexFormat::Float32x4,
            offset: 4 * 16,
            shader_location: 6,
        },
    ],
};

pub const BIND_GROUP_LAYOUT_DESCRIPTOR: BindGroupLayoutDescriptor = BindGroupLayoutDescriptor {
    label: None,
    entries: &[
        BindGroupLayoutEntry {
            binding: 0,
            visibility: ShaderStages::FRAGMENT,
            ty: BindingType::Texture {
                sample_type: TextureSampleType::Float { filterable: false },
                view_dimension: TextureViewDimension::D2,
                multisampled: false,
            },
            count: None,
        },
        BindGroupLayoutEntry {
            binding: 1,
            visibility: ShaderStages::FRAGMENT,
            ty: BindingType::Sampler(SamplerBindingType::NonFiltering),
            count: None,
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
