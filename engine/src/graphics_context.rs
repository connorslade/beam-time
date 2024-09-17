use std::rc::Rc;

use nalgebra::Vector2;
use wgpu::Color;

use crate::{
    assets::{manager::AssetManager, Asset, AssetRef},
    color::Rgb,
    render::sprite::GpuSprite,
};

pub struct GraphicsContext {
    /// Reference to asset manager
    pub(crate) asset_manager: Rc<AssetManager>,

    /// background color
    pub(crate) background: Rgb<f32>,
    /// list of sprites to render this frame
    pub(crate) sprites: Vec<GpuSprite>,

    /// Window size
    pub size: Vector2<f32>,
    /// Current window scale_factor
    pub scale_factor: f32,
    /// Mouse pos
    pub mouse: Vector2<f32>,
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

impl GraphicsContext {
    pub fn new(
        asset_manager: Rc<AssetManager>,
        size: Vector2<u32>,
        scale_factor: f32,
        mouse: Vector2<f32>,
        delta_time: f32,
    ) -> Self {
        GraphicsContext {
            asset_manager,
            background: Rgb::new(0.0, 0.0, 0.0),
            sprites: Vec::new(),
            size: size.map(|x| x as f32),
            scale_factor,
            delta_time,
            mouse,
        }
    }

    pub fn center(&self) -> Vector2<f32> {
        self.size / 2.0
    }

    pub fn background(&mut self, color: Rgb<f32>) {
        self.background = color;
    }

    pub fn draw(&mut self, drawable: impl Drawable) {
        drawable.draw(self);
    }

    pub fn get_asset(&self, asset: AssetRef) -> &Asset {
        self.asset_manager.get(asset)
    }
}

impl GraphicsContext {
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
            _ => unimplemented!(),
        }
    }
}
