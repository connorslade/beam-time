use nalgebra::{Vector2, Vector3};

use crate::{
    assets::AssetRef,
    graphics_context::{Anchor, Drawable, GraphicsContext},
    render::sprite::GpuSprite,
};

#[derive(Debug)]
pub struct Sprite {
    texture: AssetRef,

    position: Vector2<f32>,
    rotation: f32,

    anchor: Anchor,
    scale: Vector2<f32>,

    color: Vector3<f32>,
}

impl Sprite {
    pub fn new(texture: AssetRef) -> Self {
        Self {
            texture,

            position: Vector2::repeat(0.0),
            rotation: 0.0,

            anchor: Anchor::BottomLeft,
            scale: Vector2::repeat(1.0),
            color: Vector3::repeat(1.0),
        }
    }

    pub fn pos(mut self, pos: Vector2<f32>, anchor: Anchor) -> Self {
        self.position = pos;
        self.anchor = anchor;
        self
    }

    pub fn rotate(mut self, rotation: f32) -> Self {
        self.rotation = rotation;
        self
    }

    pub fn scale(mut self, scale: Vector2<f32>) -> Self {
        self.scale = scale;
        self
    }

    pub fn color(mut self, color: Vector3<f32>) -> Self {
        self.color = color;
        self
    }
}

impl Drawable for Sprite {
    fn draw(self, ctx: &mut GraphicsContext) {
        let asset = ctx
            .asset_manager
            .get(self.texture)
            .as_sprite()
            .expect("Tried to draw a font as a sprite");

        let scale = self.scale * ctx.scale_factor;

        let size = asset.size.map(|x| x as f32).component_mul(&scale);
        let pos_a = self.anchor.offset(self.position, size);
        let pos_b = pos_a + size;

        ctx.sprites.push(GpuSprite {
            texture: asset.texture,
            uv: asset.uv(),
            pos: (pos_a, pos_b),
            color: self.color,
        });
    }
}
