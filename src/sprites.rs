use nalgebra::Vector2;

use crate::assets::AssetRef;

pub struct Sprite {
    pub texture: AssetRef,
    pub pos: Vector2<u32>,
}
