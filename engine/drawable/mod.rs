use nalgebra::Vector2;

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
