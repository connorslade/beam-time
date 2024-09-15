use std::{
    hash::{Hash, Hasher},
    sync::Arc,
};

use font::FontDescriptor;
use nalgebra::Vector2;

pub mod constructor;
pub mod font;
pub mod manager;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AssetRef(u32);

pub struct Texture {
    pub texture: wgpu::Texture,
    pub size: Vector2<u32>,
}

pub enum Asset {
    Sprite(SpriteAsset),
    Font(FontAsset),
}

pub struct SpriteAsset {
    pub texture: Arc<Texture>,
    pub uv: Vector2<u32>,
    pub size: Vector2<u32>,
}

pub struct FontAsset {
    pub texture: Arc<Texture>,
    pub desc: FontDescriptor,
}

pub const fn asset(name: &str) -> AssetRef {
    let hash = const_fnv1a_hash::fnv1a_hash_str_32(name);
    AssetRef(hash)
}

impl Asset {
    pub fn as_sprite(&self) -> Option<&SpriteAsset> {
        match self {
            Asset::Sprite(sprite) => Some(sprite),
            _ => None,
        }
    }

    pub fn as_font(&self) -> Option<&FontAsset> {
        match self {
            Asset::Font(font) => Some(font),
            _ => None,
        }
    }
}

impl SpriteAsset {
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
