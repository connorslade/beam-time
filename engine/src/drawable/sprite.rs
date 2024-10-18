use nalgebra::{Matrix3, Vector2, Vector3};

use crate::{
    assets::{SpriteAsset, SpriteRef},
    color::Rgb,
    graphics_context::{Anchor, Drawable, GraphicsContext},
    render::sprite::GpuSprite,
};

#[derive(Debug)]
pub struct Sprite {
    texture: SpriteRef,
    uv_offset: Vector2<u32>,

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
    pub fn new(texture: SpriteRef) -> Self {
        Self {
            texture,
            uv_offset: Vector2::zeros(),

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
    pub fn is_hovered<App>(&self, ctx: &GraphicsContext<App>) -> bool {
        let asset = ctx.assets.get_sprite(self.texture);
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

    pub fn uv_offset(mut self, offset: Vector2<u32>) -> Self {
        self.uv_offset = offset;
        self
    }

    pub fn clamp(mut self, min: Vector2<f32>, max: Vector2<f32>) -> Self {
        // TODO: use https://github.com/ishape-rust/ioverlay
        todo!()
    }

    fn points<App>(&self, ctx: &GraphicsContext<App>, sprite: &SpriteAsset) -> [Vector2<f32>; 4] {
        let size = sprite.size.map(|x| x as f32) * ctx.scale_factor;
        let scaled_size = size.component_mul(&self.scale);

        // Calculate anchor offsets for each transformation
        let rotation_offset = self.rotation_anchor.offset(size);
        let scale_offset = self.scale_anchor.offset(size);
        let back_scale_offset = -self.scale_anchor.offset(scaled_size);
        let position_offset = self.position_anchor.offset(scaled_size);

        // Combine transformations and offsets
        let transform =
            Matrix3::new_translation(&(self.position + back_scale_offset + position_offset))
                * Matrix3::new_nonuniform_scaling(&self.scale)
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

impl<App> Drawable<App> for Sprite {
    fn draw(self, ctx: &mut GraphicsContext<App>) {
        let asset = ctx.assets.get_sprite(self.texture);

        let points = self.points(ctx, asset);
        ctx.sprites.push(GpuSprite {
            texture: asset.texture,
            uv: asset.uv(self.uv_offset),
            points,
            color: Vector3::new(self.color.r, self.color.g, self.color.b),
            z_index: self.z_index,
        });
    }
}
