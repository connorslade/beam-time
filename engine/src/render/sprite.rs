use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    AddressMode, BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout,
    BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingResource, BindingType, BlendComponent,
    BlendState, Buffer, BufferUsages, ColorTargetState, ColorWrites, Device, FilterMode,
    FragmentState, IndexFormat, MultisampleState, PipelineCompilationOptions,
    PipelineLayoutDescriptor, PrimitiveState, Queue, RenderPass, RenderPipeline,
    RenderPipelineDescriptor, Sampler, SamplerBindingType, SamplerDescriptor, ShaderStages,
    TextureSampleType, TextureViewDescriptor, TextureViewDimension, VertexState,
};

use crate::{
    assets::manager::AssetManager, graphics_context::GraphicsContext, include_shader,
    render::consts::VERTEX_BUFFER_LAYOUT, TEXTURE_FORMAT,
};

use super::Vertex;

pub struct SpriteRenderPipeline {
    render_pipeline: RenderPipeline,
    bind_group_layout: BindGroupLayout,
    bind_group: Option<BindGroup>,
    sampler: Sampler,

    vertex_buffer: Option<Buffer>,
    index_buffer: Option<Buffer>,
    index_count: u32,
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
            depth_stencil: None,
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
            bind_group: None,
            sampler,

            vertex_buffer: None,
            index_buffer: None,
            index_count: 0,
        }
    }

    pub fn prepare(
        &mut self,
        device: &Device,
        _queue: &Queue,
        assets: &AssetManager,
        ctx: &GraphicsContext,
    ) {
        let mut vert = Vec::new();
        let mut index = Vec::new();

        let size = ctx.size.map(|x| x as f32);
        for sprite in ctx.sprites.iter() {
            let asset = assets.get(sprite.asset);
            let (uv_start, uv_end) = asset.uv();
            let color = [sprite.color.x, sprite.color.y, sprite.color.z];

            let asset_size = asset.size.map(|x| x as f32).component_mul(&sprite.scale);
            let pos = sprite
                .real_pos(asset_size.map(|x| x as u32))
                .xy()
                .map(|x| x as f32)
                .component_div(&size);

            let asset_size = asset_size.component_div(&size);
            vert.extend_from_slice(&[
                Vertex::new([pos.x, pos.y, 1.0], [uv_start.x, uv_end.y], color),
                Vertex::new(
                    [pos.x + asset_size.x, pos.y, 1.0],
                    [uv_end.x, uv_end.y],
                    color,
                ),
                Vertex::new(
                    [pos.x + asset_size.x, pos.y + asset_size.y, 1.0],
                    [uv_end.x, uv_start.y],
                    color,
                ),
                Vertex::new(
                    [pos.x, pos.y + asset_size.y, 1.0],
                    [uv_start.x, uv_start.y],
                    color,
                ),
            ]);

            let base = vert.len() as u32 - 4;
            index.extend_from_slice(&[base, base + 1, base + 2, base + 2, base + 3, base]);
        }

        // todo: only re-create if changes?
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

        let view = assets
            .get(ctx.sprites[0].asset)
            .texture
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

        self.bind_group = Some(bind_group);
        self.vertex_buffer = Some(vertex_buffer);
        self.index_buffer = Some(index_buffer);
        self.index_count = index.len() as u32;
    }

    pub fn paint<'a>(&'a self, render_pass: &mut RenderPass<'a>) {
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, self.bind_group.as_ref().unwrap(), &[]);

        render_pass.set_vertex_buffer(0, self.vertex_buffer.as_ref().unwrap().slice(..));
        render_pass.set_index_buffer(
            self.index_buffer.as_ref().unwrap().slice(..),
            IndexFormat::Uint32,
        );
        render_pass.draw_indexed(0..self.index_count, 0, 0..1);
    }
}
