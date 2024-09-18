use nalgebra::{Rotation2, Vector2, Vector3};

use crate::{
    assets::AssetRef,
    color::Rgb,
    graphics_context::{Anchor, Drawable, GraphicsContext},
    render::sprite::GpuSprite,
};

#[derive(Debug)]
pub struct Sprite {
    texture: AssetRef,
    color: Rgb<f32>,

    position: Vector2<f32>,
    rotation: f32,

    scale: Vector2<f32>,
    scale_anchor: Anchor,
}

impl Sprite {
    pub fn new(texture: AssetRef) -> Self {
        Self {
            texture,
            color: Rgb::new(1.0, 1.0, 1.0),

            position: Vector2::repeat(0.0),
            rotation: 0.0,

            scale: Vector2::repeat(1.0),
            scale_anchor: Anchor::BottomLeft,
        }
    }

    pub fn is_hovered(&self, ctx: &GraphicsContext) -> bool {
        assert_eq!(
            self.rotation, 0.0,
            "Rotation is not supported for is_hovered"
        );

        let sprite = ctx.asset_manager.get(self.texture).as_sprite().unwrap();
        let scale = self.scale * ctx.scale_factor;

        let size = sprite.size.map(|x| x as f32).component_mul(&scale);
        let pos_a = self.scale_anchor.offset(self.position, size);
        let pos_b = pos_a + size;

        // check if ctx.mouse is in the rectangle
        let mouse = ctx.input.mouse;
        mouse.x >= pos_a.x && mouse.x <= pos_b.x && mouse.y <= pos_b.y && mouse.y >= pos_a.y
    }

    pub fn pos(mut self, pos: Vector2<f32>, anchor: Anchor) -> Self {
        self.position = pos;
        self.scale_anchor = anchor;
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

    pub fn color(mut self, color: impl Into<Rgb<f32>>) -> Self {
        self.color = color.into();
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
        let half_size = size / 2.0;

        let rotation = Rotation2::new(self.rotation);
        let offset = self.scale_anchor.offset(self.position, size) + half_size;

        let points = [
            rotation * half_size.component_mul(&Vector2::new(-1.0, -1.0)) + offset,
            rotation * half_size.component_mul(&Vector2::new(-1.0, 1.0)) + offset,
            rotation * half_size.component_mul(&Vector2::new(1.0, 1.0)) + offset,
            rotation * half_size.component_mul(&Vector2::new(1.0, -1.0)) + offset,
        ];

        ctx.sprites.push(GpuSprite {
            texture: asset.texture,
            uv: asset.uv(),
            points,
            color: Vector3::new(self.color.r, self.color.g, self.color.b),
        });
    }
}
