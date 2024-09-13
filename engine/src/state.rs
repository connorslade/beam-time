use std::sync::Arc;

use wgpu::{Device, Queue, Surface};
use winit::window::Window;

use crate::{
    assets::manager::AssetManager, render::sprite::SpriteRenderPipeline, screens::Screens,
};

pub struct State<'a> {
    pub graphics: RenderContext<'a>,
    pub assets: AssetManager,
    pub screens: Screens,

    pub sprite_renderer: SpriteRenderPipeline,
}

pub struct RenderContext<'a> {
    pub window: Arc<Window>,
    pub surface: Surface<'a>,
    pub device: Device,
    pub queue: Queue,
}
