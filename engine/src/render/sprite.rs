use std::collections::HashMap;

use nalgebra::{Vector2, Vector3};
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    AddressMode, BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout,
    BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingResource, BindingType, BlendComponent,
    BlendState, Buffer, BufferUsages, ColorTargetState, ColorWrites, CompareFunction,
    DepthBiasState, DepthStencilState, Device, FilterMode, FragmentState, IndexFormat,
    MultisampleState, PipelineCompilationOptions, PipelineLayoutDescriptor, PrimitiveState, Queue,
    RenderPass, RenderPipeline, RenderPipelineDescriptor, Sampler, SamplerBindingType,
    SamplerDescriptor, ShaderStages, StencilState, TextureSampleType, TextureViewDescriptor,
    TextureViewDimension, VertexState,
};

use crate::{
    assets::TextureRef, graphics_context::GraphicsContext, include_shader,
    render::consts::VERTEX_BUFFER_LAYOUT, DEPTH_TEXTURE_FORMAT, TEXTURE_FORMAT,
};

use super::Vertex;

pub struct SpriteRenderPipeline {
    render_pipeline: RenderPipeline,
    bind_group_layout: BindGroupLayout,
    sampler: Sampler,

    operations: Vec<RenderOperation>,
}

struct RenderOperation {
    bind_group: BindGroup,
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    index_count: u32,
}

#[derive(Debug)]
pub struct GpuSprite {
    pub texture: TextureRef,
    pub uv: (Vector2<f32>, Vector2<f32>),
    pub points: [Vector2<f32>; 4],
    pub color: Vector3<f32>,
    pub z_index: i16,
}

impl SpriteRenderPipeline {
    pub fn new(device: &Device) -> Self {
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
                    count: None,
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
                buffers: &[VERTEX_BUFFER_LAYOUT],
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

        let sampler = device.create_sampler(&SamplerDescriptor {
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
        });

        Self {
            render_pipeline,
            bind_group_layout,
            sampler,

            operations: Vec::new(),
        }
    }

    pub fn prepare<App>(&mut self, device: &Device, _queue: &Queue, ctx: &GraphicsContext<App>) {
        let mut atlases = HashMap::<TextureRef, Vec<&GpuSprite>>::new();

        for sprite in ctx.sprites.iter() {
            atlases.entry(sprite.texture).or_default().push(sprite);
        }

        self.operations.clear();
        for (atlas, sprites) in atlases.iter() {
            let (mut vert, mut index) = (Vec::new(), Vec::new());

            for sprite in sprites {
                let color = [sprite.color.x, sprite.color.y, sprite.color.z];
                let (uv_a, uv_b) = sprite.uv;

                let pos_a = sprite.points[0].component_div(&ctx.size());
                let pos_b = sprite.points[1].component_div(&ctx.size());
                let pos_c = sprite.points[2].component_div(&ctx.size());
                let pos_d = sprite.points[3].component_div(&ctx.size());

                let z = (i16::MAX as f32 - sprite.z_index as f32) / (i16::MAX as f32 * 2.0);

                let base = vert.len() as u32;
                vert.extend_from_slice(&[
                    Vertex::new([pos_a.x, pos_a.y, z], [uv_a.x, uv_b.y], color),
                    Vertex::new([pos_b.x, pos_b.y, z], [uv_a.x, uv_a.y], color),
                    Vertex::new([pos_c.x, pos_c.y, z], [uv_b.x, uv_a.y], color),
                    Vertex::new([pos_d.x, pos_d.y, z], [uv_b.x, uv_b.y], color),
                ]);
                index.extend_from_slice(&[base, base + 1, base + 2, base + 2, base + 3, base]);
            }

            let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
                label: None,
                usage: BufferUsages::VERTEX,
                contents: bytemuck::cast_slice(&vert),
            });

            let index_buffer = device.create_buffer_init(&BufferInitDescriptor {
                label: None,
                usage: BufferUsages::INDEX,
                contents: bytemuck::cast_slice(&index),
            });

            let texture = ctx.assets.get_texture(*atlas);
            let view = texture
                .texture
                .create_view(&TextureViewDescriptor::default());

            let bind_group = device.create_bind_group(&BindGroupDescriptor {
                label: None,
                layout: &self.bind_group_layout,
                entries: &[
                    BindGroupEntry {
                        binding: 0,
                        resource: BindingResource::TextureView(&view),
                    },
                    BindGroupEntry {
                        binding: 1,
                        resource: BindingResource::Sampler(&self.sampler),
                    },
                ],
            });

            self.operations.push(RenderOperation {
                bind_group,
                vertex_buffer,
                index_buffer,
                index_count: index.len() as u32,
            });
        }
    }

    pub fn paint<'a>(&'a self, render_pass: &mut RenderPass<'a>) {
        render_pass.set_pipeline(&self.render_pipeline);

        for operation in self.operations.iter() {
            render_pass.set_bind_group(0, &operation.bind_group, &[]);
            render_pass.set_vertex_buffer(0, operation.vertex_buffer.slice(..));
            render_pass.set_index_buffer(operation.index_buffer.slice(..), IndexFormat::Uint32);
            render_pass.draw_indexed(0..operation.index_count, 0, 0..1);
        }
    }
}
