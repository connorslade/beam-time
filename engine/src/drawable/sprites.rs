use nalgebra::{Matrix3, Vector2, Vector3};

use crate::{
    assets::{AssetRef, SpriteAsset},
    color::Rgb,
    graphics_context::{Anchor, Drawable, GraphicsContext},
    render::sprite::GpuSprite,
};

#[derive(Debug)]
pub struct Sprite {
    texture: AssetRef,
    color: Rgb<f32>,
    z_index: i16,

    position: Vector2<f32>,
    position_anchor: Anchor,

    scale: Vector2<f32>,
    scale_anchor: Anchor,

    rotation: f32,
    rotation_anchor: Anchor,
}

impl Sprite {
    pub fn new(texture: AssetRef) -> Self {
        Self {
            texture,
            color: Rgb::new(1.0, 1.0, 1.0),
            z_index: 0,

            position: Vector2::zeros(),
            position_anchor: Anchor::BottomLeft,

            scale: Vector2::repeat(1.0),
            scale_anchor: Anchor::Center,

            rotation: 0.0,
            rotation_anchor: Anchor::Center,
        }
    }

    // Reference: https://stackoverflow.com/a/37865332/12471934
    pub fn is_hovered(&self, ctx: &GraphicsContext) -> bool {
        let asset = ctx.asset_manager.get(self.texture).as_sprite().unwrap();
        let points = self.points(ctx, asset);

        let ab = points[1] - points[0];
        let am = ctx.input.mouse - points[0];
        let bc = points[2] - points[1];
        let bm = ctx.input.mouse - points[1];

        let dot_ab_am = ab.dot(&am);
        let dot_ab_ab = ab.dot(&ab);
        let dot_bc_bm = bc.dot(&bm);
        let dot_bc_bc = bc.dot(&bc);

        0.0 <= dot_ab_am && dot_ab_am <= dot_ab_ab && 0.0 <= dot_bc_bm && dot_bc_bm <= dot_bc_bc
    }

    pub fn z_index(mut self, z_index: i16) -> Self {
        self.z_index = z_index;
        self
    }

    pub fn color(mut self, color: impl Into<Rgb<f32>>) -> Self {
        self.color = color.into();
        self
    }

    pub fn position(mut self, position: Vector2<f32>, anchor: Anchor) -> Self {
        self.position = position;
        self.position_anchor = anchor;
        self
    }

    pub fn scale(mut self, scale: Vector2<f32>, anchor: Anchor) -> Self {
        self.scale = scale;
        self.scale_anchor = anchor;
        self
    }

    pub fn rotate(mut self, rotation: f32, anchor: Anchor) -> Self {
        self.rotation = rotation;
        self.rotation_anchor = anchor;
        self
    }

    fn points(&self, ctx: &GraphicsContext, sprite: &SpriteAsset) -> [Vector2<f32>; 4] {
        let size = sprite.size.map(|x| x as f32);
        let scale_factor = self.scale * ctx.scale_factor;
        let scaled_size = size.component_mul(&scale_factor);

        // Calculate anchor offsets for each transformation
        let rotation_offset = self.rotation_anchor.offset(size);
        let scale_offset = self.scale_anchor.offset(size);
        let back_scale_offset = -self.scale_anchor.offset(scaled_size);
        let position_offset = self.position_anchor.offset(scaled_size);

        // Combine transformations and offsets
        let transform =
            Matrix3::new_translation(&(self.position + back_scale_offset + position_offset))
                * Matrix3::new_nonuniform_scaling(&scale_factor)
                * Matrix3::new_translation(&(scale_offset - rotation_offset))
                * Matrix3::new_rotation(self.rotation)
                * Matrix3::new_translation(&rotation_offset);
        let transform = |point: Vector2<f32>| (transform * point.push(1.0)).xy();

        // Apply to the bounds of the sprite
        [
            transform(Vector2::new(0.0, 0.0)),
            transform(Vector2::new(0.0, size.y)),
            transform(size),
            transform(Vector2::new(size.x, 0.0)),
        ]
    }
}

impl Drawable for Sprite {
    fn draw(self, ctx: &mut GraphicsContext) {
        let asset = ctx
            .asset_manager
            .get(self.texture)
            .as_sprite()
            .expect("Tried to draw a font as a sprite");

        let points = self.points(ctx, asset);
        ctx.sprites.push(GpuSprite {
            texture: asset.texture,
            uv: asset.uv(),
            points,
            color: Vector3::new(self.color.r, self.color.g, self.color.b),
            z_index: self.z_index,
        });
    }
}
