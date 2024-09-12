use std::collections::HashMap;

use nalgebra::Vector2;
use wgpu::Texture;

use super::{Asset, AssetRef};

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

    pub fn load_image(
        &mut self,
        asset_ref: AssetRef,
        texture: Texture,
        uv: Vector2<u32>,
        size: Vector2<u32>,
    ) {
        self.assets
            .insert(asset_ref, Asset::Image { texture, uv, size });
    }
}
