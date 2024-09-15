use nalgebra::{Vector2, Vector3};

use crate::{
    assets::AssetRef,
    graphics_context::{Anchor, Drawable, GraphicsContext},
    render::sprite::GpuSprite,
};

#[derive(Debug)]
pub struct Sprite {
    pub asset: AssetRef,
    pub pos: Vector2<u32>,
    pub anchor: Anchor,
    pub scale: Vector2<f32>,
    pub color: Vector3<f32>,
}

pub struct SpriteBuilder {
    texture: AssetRef,
    pos: Vector2<u32>,
    anchor: Anchor,
    scale: Vector2<f32>,
    color: Vector3<f32>,
}

impl Sprite {
    pub fn builder(texture: AssetRef) -> SpriteBuilder {
        SpriteBuilder {
            texture,
            pos: Vector2::new(0, 0),
            anchor: Anchor::BottomLeft,
            scale: Vector2::new(1.0, 1.0),
            color: Vector3::new(1.0, 1.0, 1.0),
        }
    }

    fn real_pos(&self, size: Vector2<f32>) -> Vector2<i32> {
        self.anchor
            .offset(self.pos.map(|x| x as i32), size.map(|x| x as i32))
    }
}

impl SpriteBuilder {
    pub fn build(self) -> Sprite {
        Sprite {
            asset: self.texture,
            pos: self.pos,
            anchor: self.anchor,
            scale: self.scale,
            color: self.color,
        }
    }

    pub fn pos(mut self, pos: Vector2<u32>, anchor: Anchor) -> Self {
        self.pos = pos;
        self.anchor = anchor;
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
            .get(self.asset)
            .as_sprite()
            .expect("Tried to draw a font as a sprite");

        let size = asset.size.map(|x| x as f32).component_mul(&self.scale);
        let pos_a = self.real_pos(size).map(|x| x as f32);
        let pos_b = pos_a + size;

        ctx.sprites.push(GpuSprite {
            texture: asset.texture,
            uv: asset.uv(),
            pos: (pos_a, pos_b),
            color: self.color,
        });
    }
}

impl Drawable for SpriteBuilder {
    fn draw(self, ctx: &mut GraphicsContext) {
        self.build().draw(ctx);
    }
}
