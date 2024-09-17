use std::{rc::Rc, sync::Arc, time::Instant};

use nalgebra::Vector2;
use wgpu::{Device, Queue, Surface};
use winit::window::Window;

use crate::{
    assets::manager::AssetManager,  render::sprite::SpriteRenderPipeline,
    screens::Screens,
};

pub struct State<'a> {
    pub graphics: RenderContext<'a>,
    pub assets: Rc<AssetManager>,
    pub screens: Screens,

    pub last_frame: Instant,
    pub mouse_pos: Vector2<f32>,

    pub sprite_renderer: SpriteRenderPipeline,
}

pub struct RenderContext<'a> {
    pub window: Arc<Window>,
    pub surface: Surface<'a>,
    pub device: Device,
    pub queue: Queue,
}
