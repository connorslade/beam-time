use anyhow::Result;
use winit::event_loop::{ControlFlow, EventLoopBuilder};

use engine::application::Application;

fn main() -> Result<()> {
    let event_loop_builder = EventLoopBuilder::default().build()?;
    event_loop_builder.set_control_flow(ControlFlow::Wait);
    event_loop_builder.run_app(&mut Application::default())?;

    Ok(())
}
