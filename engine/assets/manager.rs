use std::collections::HashMap;

use nalgebra::Vector2;

use crate::audio::AudioSource;

use super::{AudioRef, FontAsset, FontRef, SpriteAsset, SpriteRef, TextureRef};

pub struct AssetManager {
    pub(crate) textures: HashMap<TextureRef, Texture>,
    pub(crate) audio: HashMap<AudioRef, AudioSource>,

    pub(crate) sprites: HashMap<SpriteRef, SpriteAsset>,
    pub(crate) fonts: HashMap<FontRef, FontAsset>,
}

pub struct Texture {
    pub texture: wgpu::Texture,
    pub size: Vector2<u32>,
}

impl AssetManager {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
            audio: HashMap::new(),
            sprites: HashMap::new(),
            fonts: HashMap::new(),
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

    pub fn get_audio(&self, asset_ref: AudioRef) -> &AudioSource {
        self.audio.get(&asset_ref).as_ref().unwrap()
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
