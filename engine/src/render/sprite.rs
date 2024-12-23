use std::{collections::HashMap, mem};

use bytemuck::NoUninit;
use encase::{ShaderType, StorageBuffer};
use nalgebra::{Matrix4, Vector2, Vector3, Vector4};
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    AddressMode, BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout,
    BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingResource, BindingType, BlendComponent,
    BlendState, Buffer, BufferAddress, BufferBinding, BufferBindingType, BufferUsages,
    ColorTargetState, ColorWrites, CompareFunction, DepthBiasState, DepthStencilState, Device,
    FilterMode, FragmentState, IndexFormat, MultisampleState, PipelineCompilationOptions,
    PipelineLayoutDescriptor, PrimitiveState, Queue, RenderPass, RenderPipeline,
    RenderPipelineDescriptor, Sampler, SamplerBindingType, SamplerDescriptor, ShaderStages,
    StencilState, TextureSampleType, TextureViewDescriptor, TextureViewDimension, VertexAttribute,
    VertexBufferLayout, VertexFormat, VertexState, VertexStepMode,
};

use crate::{
    assets::TextureRef, graphics_context::GraphicsContext, include_shader,
    render::consts::VERTEX_BUFFER_LAYOUT, DEPTH_TEXTURE_FORMAT, TEXTURE_FORMAT,
};

use super::Vertex;

const SPRITE_INSTANCE_BUFFER_LAYOUT: VertexBufferLayout = VertexBufferLayout {
    array_stride: mem::size_of::<Instance>() as BufferAddress,
    step_mode: VertexStepMode::Instance,
    attributes: &[
        VertexAttribute {
            format: VertexFormat::Float32x4,
            offset: 0,
            shader_location: 2,
        },
        VertexAttribute {
            format: VertexFormat::Float32x4,
            offset: 4 * 4,
            shader_location: 3,
        },
        VertexAttribute {
            format: VertexFormat::Float32x4,
            offset: 4 * 8,
            shader_location: 4,
        },
        VertexAttribute {
            format: VertexFormat::Float32x4,
            offset: 4 * 12,
            shader_location: 5,
        },
    ],
};

pub struct SpriteRenderPipeline {
    render_pipeline: RenderPipeline,
    bind_group_layout: BindGroupLayout,
    sampler: Sampler,

    vertex: Buffer,
    index: Buffer,

    operations: Vec<RenderOperation>,
}

#[derive(Debug)]
pub struct GpuSprite {
    pub texture: TextureRef,
    pub uv: [Vector2<f32>; 2],
    // pub points: [Vector2<f32>; 4],
    pub transform: Matrix4<f32>,
    pub color: Vector3<f32>,
    pub z_index: i16,
}

#[derive(NoUninit, Clone, Copy)]
#[repr(C)]
struct Instance {
    transform: [[f32; 4]; 4],
    // uv: Vector2<f32>,
    // uv_size: Vector2<f32>,

    // color: Vector3<f32>,
    // clip: Vector4<f32>,
}

struct RenderOperation {
    instances: Buffer,
    bind_group: BindGroup,
    instance_count: u32,
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
                buffers: &[VERTEX_BUFFER_LAYOUT, SPRITE_INSTANCE_BUFFER_LAYOUT],
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

        let vertex = [
            Vertex::new([-1.0, -1.0, 1.0, 1.0], [0.0, 1.0]),
            Vertex::new([1.0, -1.0, 1.0, 1.0], [1.0, 1.0]),
            Vertex::new([1.0, 1.0, 1.0, 1.0], [1.0, 0.0]),
            Vertex::new([-1.0, 1.0, 1.0, 1.0], [0.0, 0.0]),
        ];
        let vertex = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&vertex),
            usage: BufferUsages::VERTEX,
        });

        let index: [u32; 6] = [0, 1, 2, 2, 3, 0];
        let index = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&index),
            usage: BufferUsages::INDEX,
        });

        Self {
            render_pipeline,
            bind_group_layout,
            sampler,

            vertex,
            index,

            operations: Vec::new(),
        }
    }

    pub fn prepare<App>(&mut self, device: &Device, _queue: &Queue, ctx: &GraphicsContext<App>) {
        let mut atlases = HashMap::<TextureRef, Vec<&GpuSprite>>::new();

        for sprite in ctx.sprites.iter() {
            atlases.entry(sprite.texture).or_default().push(sprite);
        }

        let window = ctx.size();

        self.operations.clear();
        for (atlas, sprites) in atlases.iter() {
            let mut instances = Vec::new(); // todo don't realloc every frame
            for sprite in sprites.iter() {
                let size = sprite.texture.size;
                // let z = (i16::MAX as f32 - sprite.z_index as f32) / (i16::MAX as f32 * 2.0);
                instances.push(Instance {
                    transform: (Matrix4::new_nonuniform_scaling(&Vector3::new(
                        size.x as f32 / window.x,
                        size.y as f32 / window.y,
                        1.0,
                    )) * sprite.transform)
                        .into(),
                    // uv: sprite.uv[0],
                    // uv_size: sprite.uv[1] - sprite.uv[0],
                    // uv: Vector2::zeros(),
                    // uv_size: Vector2::new(1.0, 1.0),

                    // color: sprite.color,
                    // clip: Vector4::new(-1.0, -1.0, 1.0, 1.0),
                });
            }

            // todo: dont recreate buffer each frame
            let instance = device.create_buffer_init(&BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&instances),
                usage: BufferUsages::VERTEX,
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
                instances: instance,
                bind_group,
                instance_count: instances.len() as u32,
            });
        }
    }

    pub fn paint<'a>(&'a self, render_pass: &mut RenderPass<'a>) {
        render_pass.set_pipeline(&self.render_pipeline);

        for operation in self.operations.iter() {
            render_pass.set_bind_group(0, &operation.bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex.slice(..));
            render_pass.set_vertex_buffer(1, operation.instances.slice(..));
            render_pass.set_index_buffer(self.index.slice(..), IndexFormat::Uint32);
            render_pass.draw_indexed(0..6, 0, 0..operation.instance_count);
        }
    }
}
