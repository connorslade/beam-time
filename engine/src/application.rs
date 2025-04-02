use std::{iter, mem, rc::Rc, sync::Arc, time::Instant};

use anyhow::{Context, Result};
use wgpu::{
    CommandEncoderDescriptor, CompositeAlphaMode, Device, DeviceDescriptor, Extent3d, Features,
    Instance, InstanceDescriptor, Limits, LoadOp, MemoryHints, Operations, PresentMode, Queue,
    RenderPassColorAttachment, RenderPassDepthStencilAttachment, RenderPassDescriptor,
    RequestAdapterOptions, StoreOp, Surface, SurfaceConfiguration, Texture, TextureDescriptor,
    TextureDimension, TextureUsages, TextureViewDescriptor,
};
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoopBuilder},
    window::{Cursor, Window, WindowAttributes, WindowId},
};

use crate::{
    assets::{constructor::AssetConstructor, manager::AssetManager},
    audio::AudioManager,
    graphics_context::GraphicsContext,
    input::InputManager,
    render::{shape::pipeline::ShapeRenderPipeline, sprite::pipeline::SpriteRenderPipeline},
    DEPTH_TEXTURE_FORMAT, TEXTURE_FORMAT,
};

pub struct Application<'a> {
    args: ApplicationArgs,
    state: Option<State<'a>>,
}

pub struct ApplicationArgs {
    pub window_attributes: WindowAttributes,
    pub asset_constructor: Box<dyn Fn(&mut AssetConstructor)>,
    pub resumed: Box<dyn Fn() -> Box<dyn FnMut(&mut GraphicsContext)>>,
}

pub struct State<'a> {
    // Misc
    pub graphics: RenderContext<'a>,
    pub input: InputManager,

    pub assets: Rc<AssetManager>,
    pub audio: AudioManager,
    pub render: Box<dyn FnMut(&mut GraphicsContext)>,

    pub frame: u64,
    pub last_frame: Instant,
    pub last_cursor: Cursor,

    // Rendering stuff (pipelines & buffers)
    pub sprite_renderer: SpriteRenderPipeline,
    pub shape_renderer: ShapeRenderPipeline,
    pub depth_buffer: Texture,
}

pub struct RenderContext<'a> {
    pub window: Arc<Window>,
    pub surface: Surface<'a>,
    pub device: Device,
    pub queue: Queue,
}

impl Application<'_> {
    pub fn new(args: ApplicationArgs) -> Self {
        Self { args, state: None }
    }
}

impl ApplicationHandler for Application<'_> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(self.args.window_attributes.clone())
                .unwrap(),
        );
        let window_size = window.inner_size();

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
        (self.args.asset_constructor)(&mut asset_constructor);

        let assets = Rc::new(asset_constructor.into_manager(&device, &queue));
        self.state = Some(State {
            sprite_renderer: SpriteRenderPipeline::new(&device, assets.clone()),
            shape_renderer: ShapeRenderPipeline::new(&device),
            depth_buffer: create_depth_buffer(&device, window_size),
            audio: AudioManager::new_default_output(assets.clone()).unwrap(),
            assets,
            input: InputManager::new(window.inner_size()),
            graphics: RenderContext {
                surface,
                window,
                device,
                queue,
            },
            render: (self.args.resumed)(),
            frame: 0,
            last_frame: Instant::now(),
            last_cursor: Cursor::default(),
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
                    &mut state.input,
                    &state.audio,
                    delta_time,
                    state.frame,
                );
                (state.render)(&mut ctx);
                state.frame = state.frame.wrapping_add(1);

                if ctx.cursor != state.last_cursor {
                    gcx.window.set_cursor(ctx.cursor.clone());
                    state.last_cursor = mem::take(&mut ctx.cursor);
                }

                while let Some(defer) = ctx.defer.pop() {
                    (defer)(&mut ctx);
                }

                state.sprite_renderer.prepare(&gcx.device, &gcx.queue, &ctx);
                state.shape_renderer.prepare(&gcx.device, &gcx.queue, &ctx);

                let mut encoder = gcx
                    .device
                    .create_command_encoder(&CommandEncoderDescriptor::default());

                let output = gcx.surface.get_current_texture().unwrap();
                let view = output
                    .texture
                    .create_view(&TextureViewDescriptor::default());

                let depth_view = state
                    .depth_buffer
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
                        view: &depth_view,
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
                state.shape_renderer.paint(&mut render_pass);
                drop(render_pass);

                gcx.queue.submit(iter::once(encoder.finish()));

                output.present();

                state.input.on_frame_end();
                gcx.window.request_redraw();
            }
            WindowEvent::Resized(size) => {
                state.depth_buffer = create_depth_buffer(&state.graphics.device, size);
                self.resize_surface();
            }
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
                desired_maximum_frame_latency: 1,
                alpha_mode: CompositeAlphaMode::Opaque,
                view_formats: vec![],
            },
        );
    }
}

fn create_depth_buffer(device: &Device, window_size: PhysicalSize<u32>) -> Texture {
    let size = Extent3d {
        width: window_size.width,
        height: window_size.height,
        depth_or_array_layers: 1,
    };

    device.create_texture(&TextureDescriptor {
        label: None,
        size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: TextureDimension::D2,
        format: DEPTH_TEXTURE_FORMAT,
        usage: TextureUsages::RENDER_ATTACHMENT,
        view_formats: &[],
    })
}
