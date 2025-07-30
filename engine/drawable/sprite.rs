use nalgebra::{Matrix3, Vector2};

use crate::{
    assets::{SpriteAsset, SpriteRef},
    color::Rgb,
    graphics_context::{Anchor, Drawable, GraphicsContext},
    layout::{LayoutElement, bounds::Bounds2D},
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

    position: Vector2<f32>,
    rotation: f32,
    scale: Vector2<f32>,
    dynamic_scale: Vector2<f32>,

    position_anchor: Anchor,
    rotation_anchor: Anchor,
    scale_anchor: Anchor,
}

impl Sprite {
    pub fn new(texture: SpriteRef) -> Self {
        Self {
            texture,
            uv_offset: Vector2::zeros(),
            clip: [Vector2::zeros(), Vector2::repeat(f32::MAX)],

            color: Rgb::new(1.0, 1.0, 1.0),
            z_index: 0,

            position: Vector2::zeros(),
            rotation: 0.0,
            scale: Vector2::repeat(1.0),
            dynamic_scale: Vector2::repeat(1.0),

            position_anchor: Anchor::BottomLeft,
            rotation_anchor: Anchor::Center,
            scale_anchor: Anchor::Center,
        }
    }

    // Reference: https://stackoverflow.com/a/37865332/12471934
    pub fn is_hovered(&self, ctx: &GraphicsContext) -> bool {
        let asset = ctx.assets.get_sprite(self.texture);
        let points = self.points(ctx, asset, false);

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

    /// Used for temperately scaling a sprite with an anchor point calculated
    /// with the size of the sprite scaled with the normal scale. This is useful
    /// if you want to shrink a sprite around it's center on hover even if you
    /// are positioning it by it's bottom left corner.
    pub fn dynamic_scale(mut self, scale: Vector2<f32>, anchor: Anchor) -> Self {
        self.dynamic_scale = scale;
        self.scale_anchor = anchor;
        self
    }

    pub fn scale(mut self, scale: Vector2<f32>) -> Self {
        self.scale = scale;
        self.dynamic_scale = scale;
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

    fn points(
        &self,
        ctx: &GraphicsContext,
        sprite: &SpriteAsset,
        dynamic_scale: bool,
    ) -> [Vector2<f32>; 4] {
        let size = sprite.size.map(|x| x as f32) * ctx.scale_factor;
        let scaled_size = size.component_mul(&self.scale);
        let dynamic_scale = if dynamic_scale {
            self.dynamic_scale
        } else {
            self.scale
        };

        let delta_scaled_size = (size / 2.0)
            .component_mul(&(dynamic_scale - self.scale))
            .component_mul(
                &dynamic_scale.zip_map(&self.scale, |a, b| 1.0 - (a > b) as u8 as f32 * 0.5),
            );

        // Calculate anchor offsets for each transformation
        let rotation_offset = self.rotation_anchor.offset(size);
        let position_offset = self.position_anchor.offset(scaled_size);
        let scale_offset = self.scale_anchor.offset(delta_scaled_size);

        // Combine transformations and offsets
        let transform = Matrix3::new_translation(&(self.position + position_offset - scale_offset))
            * Matrix3::new_nonuniform_scaling(&dynamic_scale)
            * Matrix3::new_translation(&(-rotation_offset + scale_offset))
            * Matrix3::new_rotation(self.rotation)
            * Matrix3::new_translation(&rotation_offset);
        let transform = |point: Vector2<f32>| (transform * point.push(1.0)).xy();

        // Apply to the bounds of the sprite
        RECTANGLE_POINTS.map(|x| transform(x.component_mul(&size)))
    }
}

impl Sprite {
    pub fn get_z_index(&self) -> i16 {
        self.z_index
    }

    pub fn get_color(&self) -> Rgb<f32> {
        self.color
    }

    pub fn get_position(&self) -> (Vector2<f32>, Anchor) {
        (self.position, self.position_anchor)
    }

    pub fn get_dynamic_scale(&self) -> (Vector2<f32>, Anchor) {
        (self.dynamic_scale, self.scale_anchor)
    }

    pub fn get_scale(&self) -> Vector2<f32> {
        self.scale
    }

    pub fn get_rotation(&self) -> f32 {
        self.rotation
    }

    pub fn get_uv_offset(&self) -> Vector2<i32> {
        self.uv_offset
    }

    pub fn get_clip(&self) -> [Vector2<f32>; 2] {
        self.clip
    }
}

impl Drawable for Sprite {
    fn draw(self, ctx: &mut GraphicsContext) {
        let asset = ctx.assets.get_sprite(self.texture);

        ctx.sprites.push(GpuSprite {
            texture: asset.texture,
            uv: asset.uv(self.uv_offset).into(),
            points: self.points(ctx, asset, true),
            color: Rgb::new(self.color.r, self.color.g, self.color.b),
            z_index: self.z_index,
            clip: self.clip,
        });
    }
}

impl LayoutElement for Sprite {
    fn translate(&mut self, distance: Vector2<f32>) {
        self.position += distance;
    }

    fn bounds(&self, ctx: &mut GraphicsContext) -> Bounds2D {
        // TODO: Cache points maybe?
        let asset = ctx.assets.get_sprite(self.texture);
        let points = self.points(ctx, asset, false);
        Bounds2D::from_points(&points)
    }

    fn draw(self: Box<Self>, ctx: &mut GraphicsContext) {
        (*self).draw(ctx);
    }
}
