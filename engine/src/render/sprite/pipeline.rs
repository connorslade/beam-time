use std::{num::NonZero, rc::Rc};

use itertools::Itertools;
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingResource, BindingType, BlendComponent, BlendState, Buffer,
    BufferDescriptor, BufferUsages, ColorTargetState, ColorWrites, CompareFunction, DepthBiasState,
    DepthStencilState, Device, FragmentState, MultisampleState, PipelineCompilationOptions,
    PipelineLayoutDescriptor, PrimitiveState, RenderPipeline, RenderPipelineDescriptor,
    SamplerBindingType, ShaderStages, StencilState, TextureSampleType, TextureViewDescriptor,
    TextureViewDimension, VertexState,
};

use crate::{assets::manager::AssetManager, include_shader, DEPTH_TEXTURE_FORMAT, TEXTURE_FORMAT};

use super::{INITIAL_BUFFER_SIZE, SAMPLER_DESCRIPTOR, SPRITE_INSTANCE_BUFFER_LAYOUT};

pub struct SpriteRenderPipeline {
    pub render_pipeline: RenderPipeline,
    pub index: Buffer,

    pub bind_group: BindGroup,
    pub instances: Buffer,
    pub instance_count: u32,
}

impl SpriteRenderPipeline {
    pub fn new(device: &Device, assets: Rc<AssetManager>) -> Self {
        let shader = device.create_shader_module(include_shader!("sprite.wgsl"));

        let bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
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
                    count: Some(NonZero::new(assets.textures.len() as u32).unwrap()),
                },
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Sampler(SamplerBindingType::NonFiltering),
                    count: None,
                },
            ],
        });
        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: VertexState {
                module: &shader,
                entry_point: "vert",
                buffers: &[SPRITE_INSTANCE_BUFFER_LAYOUT],
                compilation_options: PipelineCompilationOptions::default(),
            },
            fragment: Some(FragmentState {
                module: &shader,
                entry_point: "frag",
                targets: &[Some(ColorTargetState {
                    format: TEXTURE_FORMAT,
                    blend: Some(BlendState {
                        color: BlendComponent::OVER,
                        alpha: BlendComponent::OVER,
                    }),
                    write_mask: ColorWrites::all(),
                })],
                compilation_options: PipelineCompilationOptions::default(),
            }),
            primitive: PrimitiveState::default(),
            depth_stencil: Some(DepthStencilState {
                format: DEPTH_TEXTURE_FORMAT,
                depth_write_enabled: true,
                depth_compare: CompareFunction::LessEqual,
                stencil: StencilState::default(),
                bias: DepthBiasState::default(),
            }),
            multisample: MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        let sampler = device.create_sampler(&SAMPLER_DESCRIPTOR);

        let textures = assets
            .textures
            .iter()
            .sorted_by_key(|x| x.0.reference)
            .map(|(_, value)| value.texture.create_view(&TextureViewDescriptor::default()))
            .collect::<Vec<_>>();

        let texture_refs = textures.iter().collect::<Vec<_>>();
        let bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: &bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureViewArray(&texture_refs),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::Sampler(&sampler),
                },
            ],
        });

        let instances = device.create_buffer(&BufferDescriptor {
            label: None,
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
            size: INITIAL_BUFFER_SIZE,
            mapped_at_creation: false,
        });

        let index: [u16; 6] = [0, 1, 2, 2, 3, 0];
        let index = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&index),
            usage: BufferUsages::INDEX,
        });

        Self {
            render_pipeline,
            index,

            bind_group,
            instances,
            instance_count: 0,
        }
    }
}
