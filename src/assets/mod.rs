use nalgebra::Vector2;
use wgpu::Texture;

mod manager;
mod refs;
pub use refs::*;

#[derive(PartialEq, Eq, Hash)]
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
