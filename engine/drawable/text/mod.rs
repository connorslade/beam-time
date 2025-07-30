use core::f32;
use std::cell::{Ref, RefCell};

use nalgebra::Vector2;

use crate::{
    assets::FontRef,
    color::Rgb,
    graphics_context::{Anchor, Drawable, GraphicsContext},
    layout::{Layout, LayoutElement, bounds::Bounds2D},
    render::sprite::GpuSprite,
};

use layout::TextLayout;

use super::RECTANGLE_POINTS;
mod layout;

#[derive(Clone)]
pub struct Text {
    font: FontRef,
    text: String,
    color: Rgb<f32>,
    max_width: f32,

    pos: Vector2<f32>,
    clip: [Vector2<f32>; 2],
    z_index: i16,
    scale: Vector2<f32>,
    dynamic_scale: Vector2<f32>,

    position_anchor: Anchor,
    scale_anchor: Anchor,

    layout: RefCell<Option<CachedLayout>>,
}

#[derive(Clone)]
struct CachedLayout {
    /// Size of the text bounding box when using scale.
    base: Vector2<f32>,
    /// Text layout using dynamic_scale.
    scaled: TextLayout,
}

impl Text {
    pub fn new(font: FontRef, text: impl ToString) -> Self {
        Self {
            font,
            text: text.to_string(),
            max_width: f32::MAX,

            pos: Vector2::repeat(0.0),
            clip: [Vector2::zeros(), Vector2::repeat(f32::MAX)],
            z_index: 0,
            color: Rgb::new(1.0, 1.0, 1.0),
            scale: Vector2::repeat(1.0),
            dynamic_scale: Vector2::repeat(1.0),

            position_anchor: Anchor::BottomLeft,
            scale_anchor: Anchor::Center,

            layout: RefCell::new(None),
        }
    }

    pub fn size(&self, ctx: &GraphicsContext) -> Vector2<f32> {
        self.generate_layout(ctx);
        self.layout.borrow().as_ref().unwrap().base
    }

    pub fn scaled_layout(&self, ctx: &GraphicsContext) -> Ref<'_, TextLayout> {
        self.generate_layout(ctx);
        Ref::map(self.layout.borrow(), |x| &x.as_ref().unwrap().scaled)
    }

    pub fn position(mut self, pos: Vector2<f32>, anchor: Anchor) -> Self {
        self.pos = pos;
        self.position_anchor = anchor;
        self
    }

    pub fn z_index(mut self, z_index: i16) -> Self {
        self.z_index = z_index;
        self
    }

    pub fn max_width(mut self, max_width: f32) -> Self {
        self.max_width = max_width;
        self.invalidate_layout();
        self
    }

    pub fn scale(mut self, scale: Vector2<f32>) -> Self {
        self.scale = scale;
        self.dynamic_scale = scale;
        self.invalidate_layout();
        self
    }

    pub fn dynamic_scale(mut self, scale: Vector2<f32>, anchor: Anchor) -> Self {
        self.dynamic_scale = scale;
        self.scale_anchor = anchor;
        self.invalidate_layout(); // todo: i think
        self
    }

    pub fn color(mut self, color: impl Into<Rgb<f32>>) -> Self {
        self.color = color.into();
        self
    }

    pub fn clip(mut self, a: Vector2<f32>, b: Vector2<f32>) -> Self {
        let (x1, x2) = (a.x.min(b.x), a.x.max(b.x));
        let (y1, y2) = (a.y.min(b.y), a.y.max(b.y));
        self.clip = [Vector2::new(x1, y1), Vector2::new(x2, y2)];
        self
    }

    pub fn is_hovered(&self, ctx: &mut GraphicsContext) -> bool {
        self.generate_layout(ctx);

        let layout = self.layout.borrow();
        let layout = layout.as_ref().unwrap();

        let min = self.pos + self.position_anchor.offset(layout.base);
        let max = min + layout.base;

        let mouse = ctx.input.mouse;
        mouse.x >= min.x && mouse.x <= max.x && mouse.y >= min.y && mouse.y <= max.y
    }
}

impl Text {
    pub fn get_position(&self) -> (Vector2<f32>, Anchor) {
        (self.pos, self.position_anchor)
    }

    pub fn get_z_index(&self) -> i16 {
        self.z_index
    }

    pub fn get_max_width(&self) -> f32 {
        self.max_width
    }

    pub fn get_scale(&self) -> Vector2<f32> {
        self.scale
    }

    pub fn get_dynamic_scale(&self) -> (Vector2<f32>, Anchor) {
        (self.dynamic_scale, self.scale_anchor)
    }

    pub fn get_color(&self) -> Rgb<f32> {
        self.color
    }

    pub fn get_clip(&self) -> [Vector2<f32>; 2] {
        self.clip
    }
}

impl Text {
    fn generate_layout(&self, ctx: &GraphicsContext) {
        if self.layout.borrow().is_some() {
            return;
        }

        let font = ctx.assets.get_font(self.font);
        let dynamic_scale = self.dynamic_scale * ctx.scale_factor;
        let scaled = TextLayout::generate(&font.desc, self.max_width, dynamic_scale, &self.text);

        let base = if self.scale == self.dynamic_scale {
            scaled.size
        } else {
            let scale = self.scale * ctx.scale_factor;
            TextLayout::generate(&font.desc, self.max_width, scale, &self.text).size
        };

        *self.layout.borrow_mut() = Some(CachedLayout { base, scaled });
    }

    fn invalidate_layout(&self) {
        *self.layout.borrow_mut() = None;
    }

    pub fn bounds(&self, ctx: &GraphicsContext) -> Bounds2D {
        self.generate_layout(ctx);

        let layout = self.layout.borrow();
        let layout = layout.as_ref().unwrap();

        let size = layout.base;
        let offset = self.position_anchor.offset(size);

        Bounds2D::new(self.pos + offset, self.pos + size + offset)
    }
}

impl Drawable for Text {
    fn draw(self, ctx: &mut GraphicsContext) {
        let font = ctx.assets.get_font(self.font);
        let scale = self.dynamic_scale * ctx.scale_factor;

        let atlas_size = font.texture.size.map(|x| x as f32);
        let process_uv = |uv: Vector2<u32>| uv.map(|x| x as f32).component_div(&atlas_size);

        self.generate_layout(ctx);
        let layout = self.layout.into_inner().unwrap();
        let dynamic_size = layout.scaled.size;

        let delta_scaled_size =
            (layout.base / 2.0).component_mul(&(self.dynamic_scale - self.scale));
        let dynamic_scale_offset = self.scale_anchor.offset(delta_scaled_size);

        for (character, pos) in layout.scaled.chars {
            let uv_a = process_uv(character.uv);
            let uv_b = process_uv(character.uv + character.size);

            let size = character.size.map(|x| x as f32).component_mul(&scale);
            let baseline_shift = Vector2::y() * character.baseline_shift as f32 * scale.y;
            let pos = (pos + self.pos + baseline_shift + self.position_anchor.offset(dynamic_size))
                .map(|x| x.round())
                + dynamic_scale_offset;

            ctx.sprites.push(GpuSprite {
                texture: font.texture,
                uv: [uv_a, uv_b],
                points: RECTANGLE_POINTS.map(|x| pos + x.component_mul(&size)),
                color: Rgb::new(self.color.r, self.color.g, self.color.b),
                z_index: self.z_index,
                clip: self.clip,
            });
        }
    }
}

impl LayoutElement for Text {
    fn layout(self, ctx: &mut GraphicsContext, layout: &mut dyn Layout)
    where
        Self: Sized + 'static,
    {
        self.generate_layout(ctx);
        layout.layout(ctx, Box::new(self));
    }

    fn translate(&mut self, distance: Vector2<f32>) {
        self.pos += distance;
    }

    fn bounds(&self, ctx: &mut GraphicsContext) -> Bounds2D {
        self.bounds(ctx)
    }

    fn draw(self: Box<Self>, ctx: &mut GraphicsContext) {
        (*self).draw(ctx);
    }
}
