use nalgebra::{Vector2, Vector3};
use wgpu::Color;

use crate::sprites::Sprite;

#[derive(Debug)]
pub struct GraphicsContext {
    /// One over the time since the last frame
    pub delta_time: f32,
    /// background color
    pub(crate) background: Vector3<f64>,
    /// list of sprites to render this frame
    pub(crate) sprites: Vec<Sprite>,
    /// Window size
    pub size: Vector2<u32>,
    /// Mouse pos
    pub mouse: Vector2<f32>,
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
    pub fn new(size: Vector2<u32>, mouse: Vector2<f32>, delta_time: f32) -> Self {
        GraphicsContext {
            background: Vector3::zeros(),
            sprites: Vec::new(),
            size,
            delta_time,
            mouse,
        }
    }

    pub fn center(&self) -> Vector2<u32> {
        self.size / 2
    }

    pub fn background(&mut self, color: Vector3<f64>) {
        self.background = color;
    }

    pub fn draw(&mut self, drawable: impl Drawable) {
        drawable.draw(self);
    }
}

impl GraphicsContext {
    pub(crate) fn background_color(&self) -> Color {
        Color {
            r: self.background.x,
            g: self.background.y,
            b: self.background.z,
            a: 1.0,
        }
    }
}

impl Anchor {
    pub fn offset(&self, pos: Vector2<i32>, size: Vector2<i32>) -> Vector2<i32> {
        match self {
            Anchor::BottomLeft => pos,
            Anchor::Center => pos - size / 2,
            Anchor::TopCenter => pos - Vector2::new(size.x / 2, size.y),
            _ => todo!(),
        }
    }
}
