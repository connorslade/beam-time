use std::{iter, mem, rc::Rc, sync::Arc, time::Instant};

use anyhow::{Context, Result};
use wgpu::{
    CommandEncoderDescriptor, CompositeAlphaMode, Device, DeviceDescriptor, Features, Instance,
    InstanceDescriptor, Limits, LoadOp, MemoryHints, Operations, PresentMode, Queue,
    RenderPassColorAttachment, RenderPassDescriptor, RequestAdapterOptions, StoreOp, Surface,
    SurfaceConfiguration, TextureUsages, TextureViewDescriptor,
};
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoopBuilder},
    window::{Window, WindowAttributes, WindowId},
};

use crate::{
    assets::{constructor::AssetConstructor, manager::AssetManager},
    graphics_context::GraphicsContext,
    input::InputManager,
    render::sprite::SpriteRenderPipeline,
    screens::{Screen, Screens},
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

pub struct State<'a> {
    pub graphics: RenderContext<'a>,
    pub input: InputManager,
    pub last_frame: Instant,

    pub assets: Rc<AssetManager>,
    pub screens: Screens,

    pub sprite_renderer: SpriteRenderPipeline,
}

pub struct RenderContext<'a> {
    pub window: Arc<Window>,
    pub surface: Surface<'a>,
    pub device: Device,
    pub queue: Queue,
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

        let mut asset_constructor = AssetConstructor::new();
        (self.asset_constructor)(&mut asset_constructor);

        self.state = Some(State {
            sprite_renderer: SpriteRenderPipeline::new(&device),
            assets: Rc::new(asset_constructor.into_manager(&device, &queue)),
            input: InputManager::new(window.inner_size()),
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
        let state = self.state();
        if window_id != state.graphics.window.id() {
            return;
        }

        state.input.on_window_event(&event);
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::RedrawRequested => {
                let gcx = &state.graphics;

                let delta_time = state.last_frame.elapsed().as_secs_f32();
                state.last_frame = Instant::now();

                let mut ctx = GraphicsContext::new(
                    state.assets.clone(),
                    gcx.window.scale_factor() as f32,
                    &state.input,
                    delta_time,
                );
                state.screens.render(&mut ctx);
                state.screens.pop_n(ctx.close_screen);
                state.screens.extend(mem::take(&mut ctx.next_screen));

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
    pub fn run(mut self) -> Result<()> {
        let event_loop_builder = EventLoopBuilder::default().build()?;
        event_loop_builder.set_control_flow(ControlFlow::Wait);
        event_loop_builder.run_app(&mut self)?;
        Ok(())
    }

    fn state(&mut self) -> &mut State<'a> {
        self.state.as_mut().unwrap()
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
