use std::hash::{Hash, Hasher};

use nalgebra::Vector2;
use wgpu::Texture;

mod manager;
mod refs;
pub use refs::*;

#[derive(Debug, PartialEq, Eq)]
pub struct AssetRef(u32);

pub enum Asset {
    Image {
        texture: Texture,
        uv: Vector2<u32>,
        size: Vector2<u32>,
    },
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
