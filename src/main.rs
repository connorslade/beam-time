use anyhow::Result;
use application::Application;
use winit::event_loop::{ControlFlow, EventLoopBuilder};

mod application;
mod assets;
mod consts;
mod graphics_context;
mod screens;
mod sprites;
mod state;

fn main() -> Result<()> {
    let event_loop_builder = EventLoopBuilder::default().build()?;
    event_loop_builder.set_control_flow(ControlFlow::Wait);
    event_loop_builder.run_app(&mut Application::default())?;

    Ok(())
}
