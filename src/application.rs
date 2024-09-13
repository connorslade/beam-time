use std::{iter, sync::Arc};

use anyhow::Context;
use image::ImageFormat;
use nalgebra::Vector2;
use wgpu::{
    util::{DeviceExt, TextureDataOrder},
    Color, CommandEncoderDescriptor, CompositeAlphaMode, DeviceDescriptor, Extent3d, Features,
    Instance, InstanceDescriptor, Limits, LoadOp, MemoryHints, Operations, PresentMode,
    RenderPassColorAttachment, RenderPassDescriptor, RequestAdapterOptions, StoreOp,
    SurfaceConfiguration, TextureDescriptor, TextureDimension, TextureUsages,
    TextureViewDescriptor,
};
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::{WindowAttributes, WindowId},
};

use crate::{
    assets::{manager::AssetManager, TITLE},
    consts::{DEFAULT_SIZE, TEXTURE_FORMAT},
    render::sprite::SpriteRenderPipeline,
    screens::Screens,
};
use crate::{
    graphics_context::GraphicsContext,
    state::{RenderContext, State},
};

#[derive(Default)]
pub struct Application<'a> {
    state: Option<State<'a>>,
}

impl<'a> ApplicationHandler for Application<'a> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(
                    WindowAttributes::default()
                        .with_title("Beam Time")
                        .with_inner_size(PhysicalSize::new(DEFAULT_SIZE.0, DEFAULT_SIZE.1))
                        .with_resizable(false),
                )
                .unwrap(),
        );

        let instance = Instance::new(InstanceDescriptor::default());
        let adapter =
            pollster::block_on(instance.request_adapter(&RequestAdapterOptions::default()))
                .context("Failed to create adapter")
                .unwrap();

        let surface = instance.create_surface(window.clone()).unwrap();
        let (device, queue) = pollster::block_on(adapter.request_device(
            &DeviceDescriptor {
                label: None,
                required_features: Features::default(),
                required_limits: Limits::default(),
                memory_hints: MemoryHints::Performance,
            },
            None,
        ))
        .unwrap();

        let title = image::load_from_memory_with_format(
            include_bytes!("../assets/title.png"),
            ImageFormat::Png,
        )
        .unwrap();

        fn rgb_to_bgr(mut buf: Vec<u8>) -> Vec<u8> {
            for chunk in buf.chunks_exact_mut(4) {
                chunk.swap(0, 2);
            }

            buf
        }

        let texture = device.create_texture_with_data(
            &queue,
            &TextureDescriptor {
                label: None,
                size: Extent3d {
                    width: title.width(),
                    height: title.height(),
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: TextureDimension::D2,
                format: TEXTURE_FORMAT,
                usage: TextureUsages::TEXTURE_BINDING,
                view_formats: &[],
            },
            TextureDataOrder::LayerMajor,
            &rgb_to_bgr(title.into_bytes()),
        );

        let mut assets = AssetManager::new();
        assets.register_sprite(TITLE, texture, Vector2::new(0, 0), Vector2::new(81, 20));

        self.state = Some(State {
            sprite_renderer: SpriteRenderPipeline::new(&device),

            graphics: RenderContext {
                surface,
                window,
                device,
                queue,
            },
            screens: Screens::default(),
            assets,
        });
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let state = self.state.as_mut().unwrap();
        if window_id != state.graphics.window.id() {
            return;
        }

        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                let gcx = &state.graphics;

                let size = gcx.window.inner_size();
                let mut ctx = GraphicsContext::new(Vector2::new(size.width, size.height));
                state.screens.render(&mut ctx);

                state
                    .sprite_renderer
                    .prepare(&gcx.device, &gcx.queue, &state.assets, &ctx);

                let mut encoder = gcx
                    .device
                    .create_command_encoder(&CommandEncoderDescriptor::default());

                let output = gcx.surface.get_current_texture().unwrap();
                let view = output
                    .texture
                    .create_view(&TextureViewDescriptor::default());

                let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                    label: None,
                    color_attachments: &[Some(RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: Operations {
                            load: LoadOp::Clear(ctx.background_color()),
                            store: StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                });
                state.sprite_renderer.paint(&mut render_pass);
                drop(render_pass);

                gcx.queue.submit(iter::once(encoder.finish()));

                output.present();

                gcx.window.request_redraw();
            }
            WindowEvent::Resized(..) => self.resize_surface(),
            _ => (),
        }
    }
}

impl<'a> Application<'a> {
    fn resize_surface(&mut self) {
        let state = self.state.as_mut().unwrap();
        let size = state.graphics.window.inner_size();
        state.graphics.surface.configure(
            &state.graphics.device,
            &SurfaceConfiguration {
                usage: TextureUsages::RENDER_ATTACHMENT,
                format: TEXTURE_FORMAT,
                width: size.width,
                height: size.height,
                present_mode: PresentMode::Immediate,
                desired_maximum_frame_latency: 2,
                alpha_mode: CompositeAlphaMode::Opaque,
                view_formats: vec![],
            },
        );
    }
}
