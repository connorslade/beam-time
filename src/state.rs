use std::sync::Arc;

use wgpu::{Device, Queue, Surface};
use winit::window::Window;

pub struct State<'a> {
    pub graphics: GraphicsContext<'a>,
}

pub struct GraphicsContext<'a> {
    pub window: Arc<Window>,
    pub surface: Surface<'a>,
    pub device: Device,
    pub queue: Queue,
}
