use core::f32;

use nalgebra::{Vector2, Vector3};

use crate::{
    assets::{font::FontChar, FontRef},
    color::Rgb,
    graphics_context::{Anchor, Drawable, GraphicsContext},
    render::sprite::GpuSprite,
};

pub struct Text<'a> {
    font: FontRef,
    text: &'a str,
    color: Rgb<f32>,
    max_width: f32,

    pos: Vector2<f32>,
    z_index: i16,
    anchor: Anchor,
    scale: Vector2<f32>,
}

impl<'a> Text<'a> {
    pub fn new(font: FontRef, text: &'a str) -> Self {
        Self {
            font,
            text,
            max_width: f32::MAX,

            pos: Vector2::repeat(0.0),
            z_index: 0,
            anchor: Anchor::BottomLeft,
            color: Rgb::new(1.0, 1.0, 1.0),
            scale: Vector2::repeat(1.0),
        }
    }

    pub fn width<App>(&self, ctx: &GraphicsContext<App>) -> f32 {
        let scale = self.scale * ctx.scale_factor;
        let font = ctx.assets.get_font(self.font);

        let mut width = 0.0_f32;
        let mut x = 0.0_f32;

        font.desc.process_string(self.text).for_each(|c| match c {
            FontChar::Char(c) => x += c.size.x as f32 + font.desc.tracking,
            FontChar::Space => x += font.desc.space_width,
            FontChar::Newline => {
                width = width.max(x);
                x = 0.0;
            }
        });

        width.max(x).min(self.max_width * scale.x)
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

impl<'a, App> Drawable<App> for Text<'a> {
    fn draw(self, ctx: &mut GraphicsContext<App>) {
        let font = ctx.assets.get_font(self.font);
        let scale = self.scale * ctx.scale_factor;

        let atlas_size = font.texture.size.map(|x| x as f32);
        let process_uv = |uv: Vector2<u32>| uv.map(|x| x as f32).component_div(&atlas_size);

        let mut pos = Vector2::zeros();
        let mut n = 0;
        for character in font.desc.process_string(self.text) {
            let character = match character {
                FontChar::Char(character) => character,
                FontChar::Newline => {
                    pos.y -= (font.desc.height + font.desc.leading) * scale.y;
                    pos.x = 0.0;
                    continue;
                }
                FontChar::Space => {
                    pos.x += font.desc.space_width * self.scale.x;
                    continue;
                }
            };

            let uv_a = process_uv(character.uv);
            let uv_b = process_uv(character.uv + character.size);

            let size = character.size.map(|x| x as f32).component_mul(&scale);
            let baseline_shift = character.baseline_shift as f32 * scale.y;

            ctx.sprites.push(GpuSprite {
                texture: font.texture,
                uv: (uv_a, uv_b),
                // kinda a hack
                points: [
                    size,
                    pos + Vector2::y() * baseline_shift,
                    Vector2::zeros(),
                    Vector2::zeros(),
                ],
                color: Vector3::new(self.color.r, self.color.g, self.color.b),
                z_index: self.z_index,
            });

            pos.x += (character.size.x as f32 + font.desc.tracking) * scale.x;
            n += 1;

            if pos.x > self.max_width {
                pos.y -= (font.desc.height + font.desc.leading) * scale.y;
                pos.x = 0.0;
            }
        }

        for i in ctx.sprites.len() - n..ctx.sprites.len() {
            let [size, offset, ..] = ctx.sprites[i].points;
            let offset_size = Vector2::new(pos.x, font.desc.height * scale.y);
            let pos = (self.pos + offset + self.anchor.offset(offset_size)).map(|x| x.round());

            ctx.sprites[i].points = [
                pos,
                pos + Vector2::new(0.0, size.y),
                pos + size,
                pos + Vector2::new(size.x, 0.0),
            ];
        }
    }
}
