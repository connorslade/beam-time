use nalgebra::Vector2;

use crate::{assets::AssetRef, sprites::Sprite};

pub struct GraphicsContext {
    /// list of sprites to render this frame
    sprites: Vec<Sprite>,
    /// Window size
    size: Vector2<u32>,
}

impl GraphicsContext {
    pub fn new(size: Vector2<u32>) -> Self {
        GraphicsContext {
            sprites: Vec::new(),
            size,
        }
    }

    pub fn size(&self) -> Vector2<u32> {
        self.size
    }

    pub fn center(&self) -> Vector2<u32> {
        self.size / 2
    }

    pub fn draw_sprite(&mut self, texture: AssetRef, pos: Vector2<u32>) {
        self.sprites.push(Sprite { texture, pos })
    }
}
