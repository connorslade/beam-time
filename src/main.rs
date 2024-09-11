use std::{iter, sync::Arc};

use anyhow::{Context, Result};
use consts::{DEFAULT_SIZE, TEXTURE_FORMAT};
use wgpu::{
    CommandEncoderDescriptor, CompositeAlphaMode, Device, DeviceDescriptor, Features, Instance,
    InstanceDescriptor, Limits, MemoryHints, PresentMode, Queue, RequestAdapterOptions, Surface,
    SurfaceConfiguration, TextureUsages,
};
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoopBuilder},
    window::{Window, WindowAttributes, WindowId},
};

mod consts;

fn main() -> Result<()> {
    let event_loop_builder = EventLoopBuilder::default().build()?;
    event_loop_builder.set_control_flow(ControlFlow::Wait);
    event_loop_builder.run_app(&mut Application::default())?;
    Ok(())
}

struct GraphicsContext<'a> {
    window: Arc<Window>,
    surface: Surface<'a>,
    device: Device,
    queue: Queue,
}

#[derive(Default)]
struct Application<'a> {
    state: Option<State<'a>>,
}

struct State<'a> {
    graphics: GraphicsContext<'a>,
    size: (u32, u32),
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

        self.state = Some(State {
            graphics: GraphicsContext {
                surface,
                window,
                device,
                queue,
            },
            size: DEFAULT_SIZE,
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
                let mut encoder = gcx
                    .device
                    .create_command_encoder(&CommandEncoderDescriptor::default());

                // draw game

                gcx.queue.submit(iter::once(encoder.finish()));

                let output = gcx.surface.get_current_texture().unwrap();
                output.present();

                gcx.window.request_redraw();
            }
            WindowEvent::Resized(new_size) => {
                state.size = (new_size.width, new_size.height);
                self.resize_surface();
            }
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
