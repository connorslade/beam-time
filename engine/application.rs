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
    DEPTH_TEXTURE_FORMAT, TEXTURE_FORMAT,
    assets::{constructor::AssetConstructor, manager::AssetManager},
    audio::AudioManager,
    color::Rgb,
    graphics_context::GraphicsContext,
    input::InputManager,
    memory::Memory,
    render::{shape::pipeline::ShapeRenderPipeline, sprite::pipeline::SpriteRenderPipeline},
};

type Render = Box<dyn FnMut(&mut GraphicsContext)>;

pub struct Application<'a> {
    args: ApplicationArgs,
    state: Option<State<'a>>,
}

pub struct ApplicationArgs {
    pub window_attributes: WindowAttributes,
    pub asset_constructor: Box<dyn Fn(&mut AssetConstructor)>,
    pub resumed: Box<dyn Fn() -> Render>,
    pub multisample: Option<u32>,
}

pub struct State<'a> {
    // Misc
    pub graphics: RenderContext<'a>,
    pub input: InputManager,

    pub assets: Rc<AssetManager>,
    pub audio: AudioManager,
    pub memory: Memory,
    pub render: Box<dyn FnMut(&mut GraphicsContext)>,

    pub frame: u64,
    pub last_frame: Instant,
    pub last_cursor: Cursor,
    pub vsync: bool,

    // Rendering stuff (pipelines & buffers)
    pub sprite_renderer: SpriteRenderPipeline,
    pub shape_renderer: ShapeRenderPipeline,
    pub texture: Texture,
    pub depth_texture: Texture,
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
        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter = pollster::block_on(instance.request_adapter(&RequestAdapterOptions {
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
            ..Default::default()
        }))
        .context("Failed to create adapter")
        .unwrap();

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

        let samples = self.args.multisample.unwrap_or(1);
        let assets = Rc::new(asset_constructor.into_manager(&device, &queue));
        let (texture, depth_texture) = create_textures(&device, window_size, samples);
        self.state = Some(State {
            sprite_renderer: SpriteRenderPipeline::new(&device, samples, assets.clone()),
            shape_renderer: ShapeRenderPipeline::new(&device, samples),
            texture,
            depth_texture,
            audio: AudioManager::new_default_output(assets.clone()).unwrap(),
            assets,
            memory: Memory::default(),
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
            vsync: true,
        });
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let samples = self.args.multisample.unwrap_or(1);
        let is_multisample = self.args.multisample.is_some();

        let state = self.state();

        if window_id != state.graphics.window.id() {
            return;
        }

        state.input.on_window_event(&event);
        match event {
            WindowEvent::RedrawRequested => {
                let gcx = &state.graphics;

                let scale_factor = gcx.window.scale_factor() as f32;
                let delta_time = state.last_frame.elapsed().as_secs_f32();
                state.last_frame = Instant::now();
                state.memory.garbage_collect();

                // you would think there there would be a better way...
                let mut ctx = GraphicsContext {
                    assets: state.assets.clone(),
                    audio: &state.audio,
                    memory: &mut state.memory,
                    background: Rgb::new(0.0, 0.0, 0.0),
                    sprites: Vec::new(),
                    shapes: Default::default(),
                    cursor: Cursor::default(),
                    defer: Vec::new(),
                    input: &mut state.input,
                    scale_factor,
                    delta_time,
                    frame: state.frame,
                    vsync: state.vsync,
                };

                (state.render)(&mut ctx);
                while let Some(defer) = ctx.defer.pop() {
                    (defer)(&mut ctx);
                }

                let next_vsync = ctx.vsync;
                state.frame = state.frame.wrapping_add(1);

                if ctx.cursor != state.last_cursor {
                    gcx.window.set_cursor(ctx.cursor.clone());
                    state.last_cursor = mem::take(&mut ctx.cursor);
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

                let texture = if is_multisample {
                    &state.texture.create_view(&TextureViewDescriptor::default())
                } else {
                    &view
                };

                let depth_view = state
                    .depth_texture
                    .create_view(&TextureViewDescriptor::default());

                let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                    label: None,
                    color_attachments: &[Some(RenderPassColorAttachment {
                        view: texture,
                        resolve_target: is_multisample.then_some(&view),
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

                state.input.close.then(|| event_loop.exit());
                if state.vsync != next_vsync {
                    state.vsync = next_vsync;
                    self.resize_surface();
                }
            }
            WindowEvent::Resized(size) => {
                let (texture, depth) = create_textures(&state.graphics.device, size, samples);
                state.texture = texture;
                state.depth_texture = depth;
                // On MacOS you need to manually request a redraw on window resize
                state.graphics.window.request_redraw();
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
        let present_mode = [PresentMode::AutoNoVsync, PresentMode::AutoVsync][state.vsync as usize];

        state.graphics.surface.configure(
            &state.graphics.device,
            &SurfaceConfiguration {
                usage: TextureUsages::RENDER_ATTACHMENT,
                format: TEXTURE_FORMAT,
                width: size.width,
                height: size.height,
                present_mode,
                desired_maximum_frame_latency: 1,
                alpha_mode: CompositeAlphaMode::Opaque,
                view_formats: vec![],
            },
        );
    }
}

fn create_textures(
    device: &Device,
    window_size: PhysicalSize<u32>,
    samples: u32,
) -> (Texture, Texture) {
    let size = Extent3d {
        width: window_size.width,
        height: window_size.height,
        depth_or_array_layers: 1,
    };

    let render = device.create_texture(&TextureDescriptor {
        label: None,
        size,
        mip_level_count: 1,
        sample_count: samples,
        dimension: TextureDimension::D2,
        format: TEXTURE_FORMAT,
        usage: TextureUsages::RENDER_ATTACHMENT,
        view_formats: &[],
    });

    let depth = device.create_texture(&TextureDescriptor {
        label: None,
        size,
        mip_level_count: 1,
        sample_count: samples,
        dimension: TextureDimension::D2,
        format: DEPTH_TEXTURE_FORMAT,
        usage: TextureUsages::RENDER_ATTACHMENT,
        view_formats: &[],
    });

    (render, depth)
}

impl Default for ApplicationArgs {
    fn default() -> Self {
        Self {
            window_attributes: WindowAttributes::default(),
            asset_constructor: Box::new(|_| {}),
            multisample: None,
            resumed: Box::new(|| Box::new(|_| {})),
        }
    }
}
