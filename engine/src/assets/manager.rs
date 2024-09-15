use std::collections::HashMap;

use nalgebra::Vector2;

use super::{font::FontDescriptor, Asset, AssetRef, FontAsset, SpriteAsset, TextureRef};

pub struct AssetManager {
    assets: HashMap<AssetRef, Asset>,
    textures: HashMap<TextureRef, Texture>,
}

pub struct Texture {
    pub texture: wgpu::Texture,
    pub size: Vector2<u32>,
}

impl AssetManager {
    pub fn new() -> Self {
        Self {
            assets: HashMap::new(),
            textures: HashMap::new(),
        }
    }
}

impl AssetManager {
    pub fn get(&self, asset_ref: AssetRef) -> &Asset {
        self.assets.get(&asset_ref).as_ref().unwrap()
    }

    pub fn try_get(&self, asset_ref: AssetRef) -> Option<&Asset> {
        self.assets.get(&asset_ref)
    }

    pub fn register_sprite(
        &mut self,
        asset_ref: AssetRef,
        texture: TextureRef,
        uv: Vector2<u32>,
        size: Vector2<u32>,
    ) {
        self.assets
            .insert(asset_ref, Asset::Sprite(SpriteAsset { texture, uv, size }));
    }

    pub fn register_font(
        &mut self,
        asset_ref: AssetRef,
        texture: TextureRef,
        desc: FontDescriptor,
    ) {
        self.assets
            .insert(asset_ref, Asset::Font(FontAsset { texture, desc }));
    }
}

impl AssetManager {
    pub fn get_texture(&self, texture_ref: TextureRef) -> &Texture {
        self.textures.get(&texture_ref).as_ref().unwrap()
    }

    pub fn try_get_texture(&self, texture_ref: TextureRef) -> Option<&Texture> {
        self.textures.get(&texture_ref)
    }

    pub fn register_texture(&mut self, texture: Texture) -> TextureRef {
        let texture_ref = TextureRef {
            reference: self.textures.len() as u32,
            size: texture.size,
        };

        self.textures.insert(texture_ref, texture);
        texture_ref
    }
}

impl Default for AssetManager {
    fn default() -> Self {
        Self::new()
    }
}
