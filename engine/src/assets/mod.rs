use std::{
    hash::{Hash, Hasher},
    sync::Arc,
};

use nalgebra::Vector2;

pub mod constructor;
pub mod manager;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AssetRef(u32);

pub struct Texture {
    pub texture: wgpu::Texture,
    pub size: Vector2<u32>,
}

pub struct Asset {
    pub texture: Arc<Texture>,
    pub uv: Vector2<u32>,
    pub size: Vector2<u32>,
}

pub const fn asset(name: &str) -> AssetRef {
    let hash = const_fnv1a_hash::fnv1a_hash_str_32(name);
    AssetRef(hash)
}

impl Asset {
    pub(crate) fn uv(&self) -> (Vector2<f32>, Vector2<f32>) {
        let size = self.texture.size.map(|x| x as f32);

        let start = self.uv.map(|x| x as f32).component_div(&size);
        let end = start + self.size.map(|x| x as f32).component_div(&size);

        (start, end)
    }
}

impl Hash for AssetRef {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u32(self.0);
    }
}
