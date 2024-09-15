use std::{collections::HashMap, sync::Arc};

use nalgebra::Vector2;

use super::{font::FontDescriptor, Asset, AssetRef, FontAsset, SpriteAsset, Texture};

pub struct AssetManager {
    assets: HashMap<AssetRef, Asset>,
}

impl AssetManager {
    pub fn new() -> Self {
        Self {
            assets: HashMap::new(),
        }
    }

    pub fn get(&self, asset_ref: AssetRef) -> &Asset {
        self.assets.get(&asset_ref).as_ref().unwrap()
    }

    pub fn try_get(&self, asset_ref: AssetRef) -> Option<&Asset> {
        self.assets.get(&asset_ref)
    }

    pub fn register_sprite(
        &mut self,
        asset_ref: AssetRef,
        texture: Arc<Texture>,
        uv: Vector2<u32>,
        size: Vector2<u32>,
    ) {
        self.assets
            .insert(asset_ref, Asset::Sprite(SpriteAsset { texture, uv, size }));
    }

    pub fn register_font(
        &mut self,
        asset_ref: AssetRef,
        texture: Arc<Texture>,
        desc: FontDescriptor,
    ) {
        self.assets
            .insert(asset_ref, Asset::Font(FontAsset { texture, desc }));
    }
}

impl Default for AssetManager {
    fn default() -> Self {
        Self::new()
    }
}
