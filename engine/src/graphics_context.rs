use std::rc::Rc;

use nalgebra::Vector2;
use wgpu::Color;

use crate::{
    assets::{manager::AssetManager, Asset, AssetRef},
    color::Rgb,
    input::InputManager,
    matrix_stack::MatrixStack,
    render::sprite::GpuSprite,
    screens::Screen,
};

pub struct GraphicsContext<'a> {
    /// Reference to asset manager
    pub(crate) asset_manager: Rc<AssetManager>,
    /// Matrix stack for current frame
    pub matrix: MatrixStack,

    /// background color
    pub(crate) background: Rgb<f32>,
    /// list of sprites to render this frame
    pub(crate) sprites: Vec<GpuSprite>,
    /// Screens to open for next frame
    pub(crate) next_screen: Vec<Box<dyn Screen>>,
    /// Screens to close for next_frame
    pub(crate) close_screen: usize,

    pub input: &'a InputManager,
    /// Current window scale_factor
    pub scale_factor: f32,
    /// One over the time since the last frame
    pub delta_time: f32,
}

pub trait Drawable {
    fn draw(self, ctx: &mut GraphicsContext);
}

#[derive(Debug, Copy, Clone)]
pub enum Anchor {
    TopLeft,
    TopCenter,
    TopRight,
    CenterLeft,
    Center,
    CenterRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
}

impl<'a> GraphicsContext<'a> {
    pub fn new(
        asset_manager: Rc<AssetManager>,
        scale_factor: f32,
        input: &'a InputManager,
        delta_time: f32,
    ) -> Self {
        GraphicsContext {
            asset_manager,
            matrix: MatrixStack::new(),
            background: Rgb::new(0.0, 0.0, 0.0),
            sprites: Vec::new(),
            next_screen: Vec::new(),
            close_screen: 0,
            input,
            scale_factor,
            delta_time,
        }
    }

    pub fn size(&self) -> Vector2<f32> {
        self.input.window_size.map(|x| x as f32)
    }

    pub fn center(&self) -> Vector2<f32> {
        self.size() / 2.0
    }

    pub fn background(&mut self, color: Rgb<f32>) {
        self.background = color;
    }

    pub fn push_screen(&mut self, screen: impl Screen + 'static) {
        self.next_screen.push(Box::new(screen));
    }

    pub fn pop_screen(&mut self) {
        self.close_screen += 1;
    }

    pub fn draw(&mut self, drawable: impl Drawable) {
        drawable.draw(self);
    }

    pub fn get_asset(&self, asset: AssetRef) -> &Asset {
        self.asset_manager.get(asset)
    }
}

impl<'a> GraphicsContext<'a> {
    pub(crate) fn background_color(&self) -> Color {
        Color {
            r: self.background.r as f64,
            g: self.background.g as f64,
            b: self.background.b as f64,
            a: 1.0,
        }
    }
}

impl Anchor {
    pub fn offset(&self, pos: Vector2<f32>, size: Vector2<f32>) -> Vector2<f32> {
        match self {
            Anchor::BottomLeft => pos,
            Anchor::Center => pos - size / 2.0,
            Anchor::TopCenter => pos - Vector2::new(size.x / 2.0, size.y),
            Anchor::BottomRight => pos - Vector2::new(size.x, 0.0),
            Anchor::CenterRight => pos - Vector2::new(size.x, size.y / 2.0),
            Anchor::CenterLeft => pos - Vector2::new(0.0, size.y / 2.0),
            Anchor::TopLeft => pos - Vector2::new(0.0, size.y),
            _ => unimplemented!(),
        }
    }
}
