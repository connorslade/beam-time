use nalgebra::{Vector2, Vector3};

use crate::assets::TextureRef;

pub mod consts;
pub mod pipeline;
pub mod render;

#[derive(Debug)]
pub struct GpuSprite {
    pub texture: TextureRef,

    pub points: [Vector2<f32>; 4],
    pub uv: [Vector2<f32>; 2],
    pub clip: [Vector2<f32>; 2],

    pub z_index: i16,
    pub color: Vector3<f32>,
}
