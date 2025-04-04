use core::f32;
use std::cell::RefCell;

use nalgebra::Vector2;

use crate::{
    assets::FontRef,
    color::Rgb,
    graphics_context::{Anchor, Drawable, GraphicsContext},
    render::sprite::GpuSprite,
};

use layout::TextLayout;

use super::RECTANGLE_POINTS;
mod layout;

pub struct Text<'a> {
    font: FontRef,
    text: &'a str,
    color: Rgb<f32>,
    max_width: f32,

    pos: Vector2<f32>,
    clip: [Vector2<f32>; 2],
    z_index: i16,
    anchor: Anchor,
    scale: Vector2<f32>,

    layout: RefCell<Option<TextLayout>>,
}

impl<'a> Text<'a> {
    pub fn new(font: FontRef, text: &'a str) -> Self {
        Self {
            font,
            text,
            max_width: f32::MAX,

            pos: Vector2::repeat(0.0),
            clip: [Vector2::zeros(), Vector2::repeat(f32::MAX)],
            z_index: 0,
            anchor: Anchor::BottomLeft,
            color: Rgb::new(1.0, 1.0, 1.0),
            scale: Vector2::repeat(1.0),

            layout: RefCell::new(None),
        }
    }

    pub fn size(&self, ctx: &GraphicsContext) -> Vector2<f32> {
        if let Some(layout) = &*self.layout.borrow() {
            return layout.size;
        }

        let scale = self.scale * ctx.scale_factor;
        let font = ctx.assets.get_font(self.font);
        let layout = TextLayout::generate(&font.desc, self.max_width, scale, self.text);

        let size = layout.size;
        *self.layout.borrow_mut() = Some(layout);

        size
    }

    fn invalidate_layout(&self) {
        *self.layout.borrow_mut() = None;
    }

    pub fn position(mut self, pos: Vector2<f32>, anchor: Anchor) -> Self {
        self.pos = pos;
        self.anchor = anchor;
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
        self.invalidate_layout();
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
}

impl Drawable for Text<'_> {
    fn draw(self, ctx: &mut GraphicsContext) {
        let font = ctx.assets.get_font(self.font);
        let scale = self.scale * ctx.scale_factor;

        let atlas_size = font.texture.size.map(|x| x as f32);
        let process_uv = |uv: Vector2<u32>| uv.map(|x| x as f32).component_div(&atlas_size);

        let layout = self
            .layout
            .into_inner()
            .unwrap_or_else(|| TextLayout::generate(&font.desc, self.max_width, scale, self.text));
        for (character, pos) in layout.chars {
            let uv_a = process_uv(character.uv);
            let uv_b = process_uv(character.uv + character.size);

            let size = character.size.map(|x| x as f32).component_mul(&scale);
            let baseline_shift = Vector2::y() * character.baseline_shift as f32 * scale.y;
            let pos = (pos + self.pos + baseline_shift + self.anchor.offset(layout.size))
                .map(|x| x.round());

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
