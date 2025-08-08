use nalgebra::Vector2;

use crate::graphics_context::GraphicsContext;

pub mod dummy;
pub mod shape;
pub mod spacer;
pub mod sprite;
pub mod text;

pub const RECTANGLE_POINTS: [Vector2<f32>; 4] = [
    Vector2::new(0.0, 0.0),
    Vector2::new(0.0, 1.0),
    Vector2::new(1.0, 1.0),
    Vector2::new(1.0, 0.0),
];

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

    Custom(Vector2<f32>),
}

impl Anchor {
    pub fn offset(&self, size: Vector2<f32>) -> Vector2<f32> {
        match self {
            Anchor::Custom(offset) => size.component_mul(offset),

            Anchor::CenterLeft => -Vector2::new(0.0, size.y / 2.0),
            Anchor::Center => -size / 2.0,
            Anchor::CenterRight => -Vector2::new(size.x, size.y / 2.0),

            Anchor::BottomLeft => Vector2::zeros(),
            Anchor::BottomCenter => -Vector2::new(size.x / 2.0, 0.0),
            Anchor::BottomRight => -Vector2::new(size.x, 0.0),

            Anchor::TopLeft => -Vector2::new(0.0, size.y),
            Anchor::TopCenter => -Vector2::new(size.x / 2.0, size.y),
            Anchor::TopRight => -Vector2::new(size.x, size.y),
        }
    }
}

impl<T: Drawable, const N: usize> Drawable for [T; N] {
    fn draw(self, ctx: &mut GraphicsContext) {
        self.into_iter().for_each(|x| x.draw(ctx));
    }
}
