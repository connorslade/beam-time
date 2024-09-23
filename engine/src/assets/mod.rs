use std::hash::{Hash, Hasher};

use font::FontDescriptor;
use nalgebra::Vector2;

pub mod constructor;
pub mod font;
pub mod manager;

macro_rules! define_ref {
    {$($name:ident),*} => {
        $(#[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub struct $name(u32);

        impl $name {
            pub const fn new(name: &str) -> Self {
                Self(const_fnv1a_hash::fnv1a_hash_str_32(name))
            }
        }

        impl Hash for $name {
            fn hash<H: Hasher>(&self, state: &mut H) {
                state.write_u32(self.0);
            }
        })*
    };
}

define_ref! { SpriteRef, FontRef, AudioRef }

#[derive(Debug, Copy, Clone, Eq)]
pub struct TextureRef {
    reference: u32,
    pub size: Vector2<u32>,
}

pub struct SpriteAsset {
    pub texture: TextureRef,
    pub uv: Vector2<u32>,
    pub size: Vector2<u32>,
}

pub struct FontAsset {
    pub texture: TextureRef,
    pub desc: FontDescriptor,
}

impl SpriteAsset {
    pub(crate) fn uv(&self) -> (Vector2<f32>, Vector2<f32>) {
        let size = self.texture.size.map(|x| x as f32);

        let start = self.uv.map(|x| x as f32).component_div(&size);
        let end = start + self.size.map(|x| x as f32).component_div(&size);

        (start, end)
    }
}

impl Hash for TextureRef {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u32(self.reference);
    }
}

impl PartialEq for TextureRef {
    fn eq(&self, other: &Self) -> bool {
        self.reference == other.reference
    }
}
