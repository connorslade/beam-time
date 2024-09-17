use std::{iter, rc::Rc, sync::Arc, time::Instant};

use anyhow::{Context, Result};
use nalgebra::Vector2;
use wgpu::{
    CommandEncoderDescriptor, CompositeAlphaMode, DeviceDescriptor, Features, Instance, InstanceDescriptor, Limits, LoadOp, MemoryHints, Operations, PresentMode, RenderPassColorAttachment, RenderPassDepthStencilAttachment, RenderPassDescriptor, RequestAdapterOptions, StoreOp, SurfaceConfiguration, TextureDescriptor, TextureUsages, TextureViewDescriptor
};
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoopBuilder},
    window::{WindowAttributes, WindowId},
};

use crate::{
    assets::constructor::AssetConstructor,
    graphics_context::GraphicsContext,
    render::sprite::SpriteRenderPipeline,
    screens::{Screen, Screens},
    state::{RenderContext, State},
    TEXTURE_FORMAT,
};

pub struct Application<'a> {
    window_attributes: WindowAttributes,
    screen_constructor: Box<dyn Fn() -> Box<dyn Screen>>,
    asset_constructor: Box<dyn Fn(&mut AssetConstructor)>,
    state: Option<State<'a>>,
}

pub struct ApplicationArgs {
    pub window_attributes: WindowAttributes,
    pub screen_constructor: Box<dyn Fn() -> Box<dyn Screen>>,
    pub asset_constructor: Box<dyn Fn(&mut AssetConstructor)>,
}

impl<'a> Application<'a> {
    pub fn new(args: ApplicationArgs) -> Self {
        Self {
            window_attributes: args.window_attributes,
            screen_constructor: args.screen_constructor,
            asset_constructor: args.asset_constructor,
            state: None,
        }
    }
}

impl<'a> ApplicationHandler for Application<'a> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(self.window_attributes.clone())
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

        let depth_texture = device.create_texture(&TextureDescriptor {
            label: None,
            size: (),
            mip_level_count: (),
            sample_count: (),
            dimension: (),
            format: (),
            usage: (),
            view_formats: (),
        });

        let mut asset_constructor = AssetConstructor::new();
        (self.asset_constructor)(&mut asset_constructor);

        self.state = Some(State {
            sprite_renderer: SpriteRenderPipeline::new(&device),
            assets: Rc::new(asset_constructor.into_manager(&device, &queue)),
            mouse_pos: Vector2::zeros(),
            graphics: RenderContext {
                surface,
                window,
                device,
                queue,
            },
            screens: Screens::new((self.screen_constructor)()),
            last_frame: Instant::now(),
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
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::CursorMoved { position, .. } => {
                let screen_size = state.graphics.window.inner_size();
                state.mouse_pos = Vector2::new(
                    position.x as f32,
                    screen_size.height as f32 - position.y as f32,
                )
            }
            WindowEvent::RedrawRequested => {
                let gcx = &state.graphics;

                let delta_time = state.last_frame.elapsed().as_secs_f32();
                state.last_frame = Instant::now();

                let size = gcx.window.inner_size();
                let mut ctx = GraphicsContext::new(
                    state.assets.clone(),
                    Vector2::new(size.width, size.height),
                    gcx.window.scale_factor() as f32,
                    state.mouse_pos,
                    delta_time,
                );
                state.screens.render(&mut ctx);

                state.sprite_renderer.prepare(&gcx.device, &gcx.queue, &ctx);

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
                    depth_stencil_attachment: Some(RenderPassDepthStencilAttachment {
                        view: depth_texture_view,
                        depth_ops: Some(Operations {
                            load: LoadOp::Clear(1.0),
                            store: StoreOp::Store,
                        }),
                        stencil_ops: None,
                    }),
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
            WindowEvent::KeyboardInput { event, .. } => state.screens.on_key(event),
            WindowEvent::MouseInput {
                state: element_state,
                button,
                ..
            } => state.screens.on_click(element_state, button),
            _ => (),
        }
    }
}

impl<'a> Application<'a> {
    pub fn run(mut self) -> Result<()> {
        let event_loop_builder = EventLoopBuilder::default().build()?;
        event_loop_builder.set_control_flow(ControlFlow::Wait);
        event_loop_builder.run_app(&mut self)?;
        Ok(())
    }

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
                present_mode: PresentMode::AutoVsync,
                desired_maximum_frame_latency: 2,
                alpha_mode: CompositeAlphaMode::Opaque,
                view_formats: vec![],
            },
        );
    }
}
