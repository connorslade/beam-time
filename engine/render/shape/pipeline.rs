use wgpu::{
    BindGroup, BindGroupLayoutDescriptor, BlendState, Buffer, BufferDescriptor, BufferUsages,
    ColorTargetState, ColorWrites, CompareFunction, DepthBiasState, DepthStencilState, Device,
    FragmentState, MultisampleState, PipelineCompilationOptions, PipelineLayoutDescriptor,
    PrimitiveState, RenderPipeline, RenderPipelineDescriptor, StencilState, VertexAttribute,
    VertexBufferLayout, VertexFormat, VertexState, VertexStepMode,
};

use crate::{include_shader, DEPTH_TEXTURE_FORMAT, TEXTURE_FORMAT};

pub const DEFAULT_BUFFER_SIZE: u64 = 1024;

pub struct ShapeRenderPipeline {
    pub render_pipeline: RenderPipeline,
    pub bind_group: BindGroup,

    pub vertex: Buffer,
    pub index: Buffer,
    pub count: u32,
}

impl ShapeRenderPipeline {
    pub fn new(device: &Device, samples: u32) -> Self {
        let shader = device.create_shader_module(include_shader!("shape.wgsl"));

        let bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: None,
            entries: &[],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &bind_group_layout,
            entries: &[],
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
                buffers: &[VertexBufferLayout {
                    array_stride: 24,
                    step_mode: VertexStepMode::Vertex,
                    attributes: &[
                        VertexAttribute {
                            format: VertexFormat::Float32x3,
                            offset: 0,
                            shader_location: 0,
                        },
                        VertexAttribute {
                            format: VertexFormat::Float32x3,
                            offset: 4 * 3,
                            shader_location: 1,
                        },
                    ],
                }],
                compilation_options: PipelineCompilationOptions::default(),
            },
            fragment: Some(FragmentState {
                module: &shader,
                entry_point: "frag",
                targets: &[Some(ColorTargetState {
                    format: TEXTURE_FORMAT,
                    blend: Some(BlendState::PREMULTIPLIED_ALPHA_BLENDING),
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

        let vertex = device.create_buffer(&BufferDescriptor {
            label: None,
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
            size: DEFAULT_BUFFER_SIZE,
            mapped_at_creation: false,
        });

        let index = device.create_buffer(&BufferDescriptor {
            label: None,
            usage: BufferUsages::INDEX | BufferUsages::COPY_DST,
            size: DEFAULT_BUFFER_SIZE,
            mapped_at_creation: false,
        });

        Self {
            render_pipeline,
            bind_group,

            vertex,
            index,
            count: 0,
        }
    }
}
