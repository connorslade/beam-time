use std::sync::Arc;

use image::RgbaImage;
use nalgebra::Vector2;
use wgpu::{
    util::{DeviceExt, TextureDataOrder},
    Device, Extent3d, Queue, TextureDescriptor, TextureDimension, TextureUsages,
};

use crate::TEXTURE_FORMAT;

use super::{manager::AssetManager, AssetRef};

pub struct AssetConstructor {
    next_id: u32,
    atlas: Vec<RgbaImage>,
    sprites: Vec<(AtlasRef, AssetRef, LocalSprite)>,
}

pub struct AtlasRef(u32);

/// Representation of a sprite before the texture is sent to the GPU
struct LocalSprite {
    uv: Vector2<u32>,
    size: Vector2<u32>,
}

impl AssetConstructor {
    pub fn new() -> Self {
        Self {
            next_id: 0,
            atlas: Vec::new(),
            sprites: Vec::new(),
        }
    }

    pub fn register_atlas(&mut self, image: RgbaImage) -> AtlasRef {
        let id = self.next_id;
        self.next_id += 1;

        self.atlas.push(image);

        AtlasRef(id)
    }

    pub fn register_sprite(
        &mut self,
        atlas: AtlasRef,
        asset: AssetRef,
        uv: (u32, u32),
        size: (u32, u32),
    ) {
        self.sprites.push((
            atlas,
            asset,
            LocalSprite {
                uv: Vector2::new(uv.0, uv.1),
                size: Vector2::new(size.0, size.1),
            },
        ));
    }

    pub(crate) fn into_manager(self, device: &Device, queue: &Queue) -> AssetManager {
        // Upload atlases to the GPU
        let mut textures = Vec::new();
        for atlas in self.atlas {
            let texture = device.create_texture_with_data(
                &queue,
                &TextureDescriptor {
                    label: None,
                    size: Extent3d {
                        width: atlas.width(),
                        height: atlas.height(),
                        depth_or_array_layers: 1,
                    },
                    mip_level_count: 1,
                    sample_count: 1,
                    dimension: TextureDimension::D2,
                    format: TEXTURE_FORMAT,
                    usage: TextureUsages::TEXTURE_BINDING,
                    view_formats: &[],
                },
                TextureDataOrder::LayerMajor,
                &rgb_to_bgr(atlas.into_vec()),
            );

            textures.push(Arc::new(texture));
        }

        let mut manager = AssetManager::new();
        for (atlas, asset, sprite) in self.sprites {
            let texture = textures[atlas.0 as usize].clone();
            manager.register_sprite(asset, texture, sprite.uv, sprite.size);
        }

        manager
    }
}

fn rgb_to_bgr(mut buf: Vec<u8>) -> Vec<u8> {
    buf.chunks_exact_mut(4).for_each(|chunk| chunk.swap(0, 2));
    buf
}
