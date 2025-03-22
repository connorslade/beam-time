use nalgebra::{Matrix3, Vector2, Vector3};

use crate::{
    assets::{SpriteAsset, SpriteRef},
    color::Rgb,
    graphics_context::{Anchor, Drawable, GraphicsContext},
    render::sprite::GpuSprite,
};

use super::RECTANGLE_POINTS;

#[derive(Debug, Clone)]
pub struct Sprite {
    texture: SpriteRef,
    uv_offset: Vector2<i32>,
    clip: [Vector2<f32>; 2],

    color: Rgb<f32>,
    z_index: i16,

    scale: Vector2<f32>,
    position: Vector2<f32>,
    rotation: f32,

    position_anchor: Anchor,
    rotation_anchor: Anchor,
}

impl Sprite {
    pub fn new(texture: SpriteRef) -> Self {
        Self {
            texture,
            uv_offset: Vector2::zeros(),
            clip: [Vector2::zeros(), Vector2::repeat(f32::MAX)],

            color: Rgb::new(1.0, 1.0, 1.0),
            z_index: 0,

            scale: Vector2::repeat(1.0),
            position: Vector2::zeros(),
            rotation: 0.0,

            position_anchor: Anchor::BottomLeft,
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

    pub fn scale(mut self, scale: Vector2<f32>) -> Self {
        self.scale = scale;
        self
    }

    pub fn rotate(mut self, rotation: f32, anchor: Anchor) -> Self {
        self.rotation = rotation;
        self.rotation_anchor = anchor;
        self
    }

    pub fn uv_offset(mut self, offset: Vector2<i32>) -> Self {
        self.uv_offset = offset;
        self
    }

    pub fn clip(mut self, a: Vector2<f32>, b: Vector2<f32>) -> Self {
        let (x1, x2) = (a.x.min(b.x), a.x.max(b.x));
        let (y1, y2) = (a.y.min(b.y), a.y.max(b.y));
        self.clip = [Vector2::new(x1, y1), Vector2::new(x2, y2)];
        self
    }

    fn points<App>(&self, ctx: &GraphicsContext<App>, sprite: &SpriteAsset) -> [Vector2<f32>; 4] {
        let size = sprite.size.map(|x| x as f32) * ctx.scale_factor;
        let scaled_size = size.component_mul(&self.scale);

        // Calculate anchor offsets for each transformation
        let rotation_offset = self.rotation_anchor.offset(size);
        let position_offset = self.position_anchor.offset(scaled_size);

        // Combine transformations and offsets
        let transform = Matrix3::new_translation(&(self.position + position_offset))
            * Matrix3::new_nonuniform_scaling(&self.scale)
            * Matrix3::new_translation(&(-rotation_offset))
            * Matrix3::new_rotation(self.rotation)
            * Matrix3::new_translation(&rotation_offset);
        let transform = |point: Vector2<f32>| (transform * point.push(1.0)).xy();

        // Apply to the bounds of the sprite
        RECTANGLE_POINTS.map(|x| transform(x.component_mul(&size)))
    }
}

impl<App> Drawable<App> for Sprite {
    fn draw(self, ctx: &mut GraphicsContext<App>) {
        let asset = ctx.assets.get_sprite(self.texture);

        ctx.sprites.push(GpuSprite {
            texture: asset.texture,
            uv: asset.uv(self.uv_offset).into(),
            points: self.points(ctx, asset),
            color: Vector3::new(self.color.r, self.color.g, self.color.b),
            z_index: self.z_index,
            clip: self.clip,
        });
    }
}
