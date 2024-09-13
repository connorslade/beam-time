use std::{
    hash::{Hash, Hasher},
    sync::Arc,
};

use nalgebra::Vector2;
use wgpu::Texture;

pub mod constructor;
pub mod manager;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AssetRef(u32);

pub struct Asset {
    pub texture: Arc<Texture>,
    pub uv: Vector2<u32>,
    pub size: Vector2<u32>,
}

pub const fn asset(name: &str) -> AssetRef {
    let hash = const_fnv1a_hash::fnv1a_hash_str_32(name);
    AssetRef(hash)
}

impl Hash for AssetRef {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u32(self.0);
    }
}
