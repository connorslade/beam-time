use std::{sync::Arc, time::Instant};

use wgpu::{Device, Queue, Surface};
use winit::window::Window;

use crate::{
    assets::manager::AssetManager, input::InputManager, render::sprite::SpriteRenderPipeline,
    screens::Screens,
};

pub struct State<'a> {
    pub graphics: RenderContext<'a>,
    pub assets: AssetManager,
    pub screens: Screens,

    pub last_frame: Instant,
    pub input: InputManager,

    pub sprite_renderer: SpriteRenderPipeline,
}

pub struct RenderContext<'a> {
    pub window: Arc<Window>,
    pub surface: Surface<'a>,
    pub device: Device,
    pub queue: Queue,
}
