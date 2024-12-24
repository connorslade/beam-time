use std::{collections::HashMap, mem, rc::Rc};

use bytemuck::NoUninit;
use nalgebra::{Vector2, Vector3};
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    AddressMode, BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingResource, BindingType, BlendComponent, BlendState, Buffer,
    BufferAddress, BufferDescriptor, BufferUsages, ColorTargetState, ColorWrites, CompareFunction,
    DepthBiasState, DepthStencilState, Device, FilterMode, FragmentState, IndexFormat,
    MultisampleState, PipelineCompilationOptions, PipelineLayoutDescriptor, PrimitiveState, Queue,
    RenderPass, RenderPipeline, RenderPipelineDescriptor, SamplerBindingType, SamplerDescriptor,
    ShaderStages, StencilState, TextureSampleType, TextureViewDescriptor, TextureViewDimension,
    VertexAttribute, VertexBufferLayout, VertexFormat, VertexState, VertexStepMode,
};

use crate::{
    assets::{manager::AssetManager, TextureRef},
    graphics_context::GraphicsContext,
    include_shader, DEPTH_TEXTURE_FORMAT, TEXTURE_FORMAT,
};

const SPRITE_INSTANCE_BUFFER_LAYOUT: VertexBufferLayout = VertexBufferLayout {
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

const INITIAL_BUFFER_SIZE: u64 = 2 << 11;

pub struct SpriteRenderPipeline {
    render_pipeline: RenderPipeline,
    index: Buffer,

    atlases: HashMap<TextureRef, RenderOperation>,
}

#[derive(Debug)]
pub struct GpuSprite {
    pub texture: TextureRef,
    pub uv: [Vector2<f32>; 2],
    pub points: [Vector2<f32>; 4],
    pub color: Vector3<f32>,
    pub z_index: i16,
}

#[derive(NoUninit, Clone, Copy)]
#[repr(C)]
struct Instance {
    points: [[f32; 2]; 4],
    uv: [[f32; 2]; 2],
    layer: f32,
    color: [f32; 3],
    clip: [f32; 4],
}

struct RenderOperation {
    bind_group: BindGroup,
    instances: Buffer,
    instance_count: u32,
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

        let index: [u32; 6] = [0, 1, 2, 2, 3, 0];
        let index = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&index),
            usage: BufferUsages::INDEX,
        });

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

        Self {
            render_pipeline,
            index,
            atlases,
        }
    }

    pub fn prepare<App>(&mut self, device: &Device, queue: &Queue, ctx: &GraphicsContext<App>) {
        let mut atlases = HashMap::<TextureRef, Vec<&GpuSprite>>::new();

        for sprite in ctx.sprites.iter() {
            atlases.entry(sprite.texture).or_default().push(sprite);
        }

        // clear atlas lists
        for val in self.atlases.values_mut() {
            val.instance_count = 0;
        }

        for (atlas, sprites) in atlases.iter() {
            let mut instances = Vec::new(); // todo don't realloc every frame
            for sprite in sprites.iter() {
                let layer = (i16::MAX as f32 - sprite.z_index as f32) / (i16::MAX as f32 * 2.0);
                instances.push(Instance {
                    points: [
                        sprite.points[0].component_div(&ctx.size()).into(),
                        sprite.points[1].component_div(&ctx.size()).into(),
                        sprite.points[2].component_div(&ctx.size()).into(),
                        sprite.points[3].component_div(&ctx.size()).into(),
                    ],
                    layer,
                    uv: [sprite.uv[0].into(), (sprite.uv[1] - sprite.uv[0]).into()],
                    color: sprite.color.into(),
                    clip: [-1.0, -1.0, 1.0, 1.0],
                });
            }

            let contents = bytemuck::cast_slice(&instances);
            let content_len = contents.len() as u64;

            let operation = self.atlases.get_mut(atlas).unwrap();

            if content_len > operation.instances.size() {
                let size = content_len.next_power_of_two();
                operation.instances = device.create_buffer(&BufferDescriptor {
                    label: None,
                    usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
                    mapped_at_creation: false,
                    size,
                });
            }

            queue.write_buffer(&operation.instances, 0, contents);
            operation.instance_count = instances.len() as u32;
        }
    }

    pub fn paint<'a>(&'a self, render_pass: &mut RenderPass<'a>) {
        render_pass.set_pipeline(&self.render_pipeline);

        for operation in self.atlases.values() {
            render_pass.set_bind_group(0, &operation.bind_group, &[]);
            render_pass.set_vertex_buffer(0, operation.instances.slice(..));
            render_pass.set_index_buffer(self.index.slice(..), IndexFormat::Uint32);
            render_pass.draw_indexed(0..6, 0, 0..operation.instance_count);
        }
    }
}
