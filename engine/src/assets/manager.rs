use std::collections::HashMap;

use nalgebra::Vector2;

use super::{font::FontDescriptor, FontAsset, FontRef, SpriteAsset, SpriteRef, TextureRef};

pub struct AssetManager {
    sprites: HashMap<SpriteRef, SpriteAsset>,
    fonts: HashMap<FontRef, FontAsset>,
    textures: HashMap<TextureRef, Texture>,
}

pub struct Texture {
    pub texture: wgpu::Texture,
    pub size: Vector2<u32>,
}

impl AssetManager {
    pub fn new() -> Self {
        Self {
            sprites: HashMap::new(),
            fonts: HashMap::new(),
            textures: HashMap::new(),
        }
    }
}

impl AssetManager {
    pub fn get_sprite(&self, asset_ref: SpriteRef) -> &SpriteAsset {
        self.sprites.get(&asset_ref).as_ref().unwrap()
    }

    pub fn get_font(&self, asset_ref: FontRef) -> &FontAsset {
        self.fonts.get(&asset_ref).as_ref().unwrap()
    }

    pub fn register_sprite(
        &mut self,
        asset_ref: SpriteRef,
        texture: TextureRef,
        uv: Vector2<u32>,
        size: Vector2<u32>,
    ) {
        self.sprites
            .insert(asset_ref, SpriteAsset { texture, uv, size });
    }

    pub fn register_font(&mut self, asset_ref: FontRef, texture: TextureRef, desc: FontDescriptor) {
        self.fonts.insert(asset_ref, FontAsset { texture, desc });
    }
}

impl AssetManager {
    pub fn get_texture(&self, texture_ref: TextureRef) -> &Texture {
        self.textures.get(&texture_ref).as_ref().unwrap()
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
