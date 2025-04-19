use std::{collections::HashMap, rc::Rc};

use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindingResource, BlendComponent, BlendState,
    Buffer, BufferDescriptor, BufferUsages, ColorTargetState, ColorWrites, CompareFunction,
    DepthBiasState, DepthStencilState, Device, FragmentState, MultisampleState,
    PipelineCompilationOptions, PipelineLayoutDescriptor, PrimitiveState, RenderPipeline,
    RenderPipelineDescriptor, StencilState, TextureViewDescriptor, VertexState,
};

use crate::{
    assets::{manager::AssetManager, TextureRef},
    include_shader, DEPTH_TEXTURE_FORMAT, TEXTURE_FORMAT,
};

use super::consts::{
    BIND_GROUP_LAYOUT_DESCRIPTOR, INITIAL_BUFFER_SIZE, SAMPLER_DESCRIPTOR,
    SPRITE_INSTANCE_BUFFER_LAYOUT,
};

pub struct SpriteRenderPipeline {
    pub render_pipeline: RenderPipeline,
    pub index: Buffer,

    pub atlases: HashMap<TextureRef, RenderOperation>,
}

pub struct RenderOperation {
    pub bind_group: BindGroup,
    pub instances: Buffer,
    pub instance_count: u32,
}

impl SpriteRenderPipeline {
    pub fn new(device: &Device, samples: u32, assets: Rc<AssetManager>) -> Self {
        let shader = device.create_shader_module(include_shader!("sprite.wgsl"));

        let bind_group_layout = device.create_bind_group_layout(&BIND_GROUP_LAYOUT_DESCRIPTOR);
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
            multisample: MultisampleState {
                count: samples,
                ..Default::default()
            },
            multiview: None,
            cache: None,
        });

        let sampler = device.create_sampler(&SAMPLER_DESCRIPTOR);

        let mut atlases = HashMap::new();
        for (key, value) in assets.textures.iter() {
            let view = value.texture.create_view(&TextureViewDescriptor::default());

            let bind_group = device.create_bind_group(&BindGroupDescriptor {
                label: None,
                layout: &bind_group_layout,
                entries: &[
                    BindGroupEntry {
                        binding: 0,
                        resource: BindingResource::TextureView(&view),
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

            atlases.insert(
                *key,
                RenderOperation {
                    bind_group,
                    instances,
                    instance_count: 0,
                },
            );
        }

        let index: [u16; 6] = [0, 1, 2, 2, 3, 0];
        let index = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&index),
            usage: BufferUsages::INDEX,
        });

        Self {
            render_pipeline,
            index,
            atlases,
        }
    }
}
