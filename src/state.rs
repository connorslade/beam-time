use std::sync::Arc;

use wgpu::{Device, Queue, Surface};
use winit::window::Window;

use crate::screens::{Screen, Screens};

pub struct State<'a> {
    pub graphics: GraphicsContext<'a>,
    pub screens: Screens,
}

pub struct GraphicsContext<'a> {
    pub window: Arc<Window>,
    pub surface: Surface<'a>,
    pub device: Device,
    pub queue: Queue,
}
