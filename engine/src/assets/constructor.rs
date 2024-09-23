use std::io::Cursor;

use image::RgbaImage;
use nalgebra::Vector2;
use rodio::Source;
use wgpu::{
    util::{DeviceExt, TextureDataOrder},
    Device, Extent3d, Queue, TextureDescriptor, TextureDimension, TextureUsages,
};

use crate::TEXTURE_FORMAT;

use super::{
    font::FontDescriptor,
    manager::{AssetManager, Texture},
    AssetRef,
};

pub struct AssetConstructor {
    next_id: u32,
    atlas: Vec<RgbaImage>,

    audio: Vec<(AssetRef, Box<dyn Source<Item = i16>>)>,
    sprites: Vec<(AtlasRef, AssetRef, LocalSprite)>,
    fonts: Vec<(AtlasRef, AssetRef, FontDescriptor)>,
}

#[derive(Copy, Clone)]
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

            audio: Vec::new(),
            sprites: Vec::new(),
            fonts: Vec::new(),
        }
    }

    pub fn register_atlas(&mut self, image: RgbaImage) -> AtlasRef {
        let id = self.next_id;
        self.next_id += 1;

        self.atlas.push(image);

        AtlasRef(id)
    }

    pub fn register_audio(&mut self, asset: AssetRef, bytes: &'static [u8]) {
        let source = rodio::Decoder::new(Cursor::new(bytes)).unwrap();
        self.audio.push((asset, Box::new(source)));
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

    pub fn register_font(
        &mut self,
        atlas: AtlasRef,
        asset: AssetRef,
        font_descriptor: FontDescriptor,
    ) {
        self.fonts.push((atlas, asset, font_descriptor));
    }

    pub(crate) fn into_manager(self, device: &Device, queue: &Queue) -> AssetManager {
        let mut manager = AssetManager::new();

        // Upload atlases to the GPU
        let mut textures = Vec::new();
        for atlas in self.atlas {
            let size = Vector2::new(atlas.width(), atlas.height());
            let texture = device.create_texture_with_data(
                queue,
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

            let texture_ref = manager.register_texture(Texture { texture, size });
            textures.push(texture_ref);
        }

        for (atlas, asset, sprite) in self.sprites {
            let texture = textures[atlas.0 as usize];
            manager.register_sprite(asset, texture, sprite.uv, sprite.size);
        }

        for (atlas, asset, descriptor) in self.fonts {
            let texture = textures[atlas.0 as usize];
            manager.register_font(asset, texture, descriptor);
        }

        manager
    }
}

fn rgb_to_bgr(mut buf: Vec<u8>) -> Vec<u8> {
    buf.chunks_exact_mut(4).for_each(|chunk| chunk.swap(0, 2));
    buf
}

impl Default for AssetConstructor {
    fn default() -> Self {
        Self::new()
    }
}
