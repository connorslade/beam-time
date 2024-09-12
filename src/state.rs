use std::sync::Arc;

use wgpu::{Device, Queue, Surface};
use winit::window::Window;

use crate::screens::Screens;

pub struct State<'a> {
    pub graphics: RenderContext<'a>,
    pub screens: Screens,
}

pub struct RenderContext<'a> {
    pub window: Arc<Window>,
    pub surface: Surface<'a>,
    pub device: Device,
    pub queue: Queue,
}
