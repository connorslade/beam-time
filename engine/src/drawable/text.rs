use nalgebra::{Vector2, Vector3};

use crate::{
    assets::{font::FontChar, AssetRef},
    color::Rgb,
    graphics_context::{Anchor, Drawable, GraphicsContext},
    render::sprite::GpuSprite,
};

pub struct Text<'a> {
    font: AssetRef,
    text: &'a str,
    color: Rgb<f32>,

    pos: Vector2<f32>,
    anchor: Anchor,
    scale: Vector2<f32>,
}

impl<'a> Text<'a> {
    pub fn new(font: AssetRef, text: &'a str) -> Self {
        Self {
            font,
            text,

            pos: Vector2::repeat(0.0),
            anchor: Anchor::BottomLeft,
            color: Rgb::new(1.0, 1.0, 1.0),
            scale: Vector2::repeat(1.0),
        }
    }

    pub fn width(&self, ctx: &GraphicsContext) -> f32 {
        let scale = self.scale * ctx.scale_factor;
        let font = ctx
            .asset_manager
            .get(self.font)
            .as_font()
            .expect("Tried to use an non-font asset as a font.");

        font.desc
            .process_string(self.text)
            .map(|c| match c {
                FontChar::Char(c) => c.size.x as f32 + font.desc.tracking,
                FontChar::Space => font.desc.space_width,
            })
            .sum::<f32>()
            * scale.x
    }

    pub fn pos(mut self, pos: Vector2<f32>, anchor: Anchor) -> Self {
        self.pos = pos;
        self.anchor = anchor;
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

impl<'a> Drawable for Text<'a> {
    fn draw(self, ctx: &mut GraphicsContext) {
        let font = ctx
            .asset_manager
            .get(self.font)
            .as_font()
            .expect("Tried to use an non-font asset as a font.");

        let scale = self.scale * ctx.scale_factor;

        let atlas_size = font.texture.size.map(|x| x as f32);
        let process_uv = |uv: Vector2<u32>| uv.map(|x| x as f32).component_div(&atlas_size);

        let mut x = 0.0;
        let mut n = 0;
        for character in font.desc.process_string(self.text) {
            let character = match character {
                FontChar::Char(character) => character,
                FontChar::Space => {
                    x += font.desc.space_width * self.scale.x;
                    continue;
                }
            };

            let uv_a = process_uv(character.uv);
            let uv_b = process_uv(character.uv + character.size);

            let size = character.size.map(|x| x as f32).component_mul(&scale);

            ctx.sprites.push(GpuSprite {
                texture: font.texture,
                uv: (uv_a, uv_b),
                // kinda a hack
                points: [
                    size,
                    Vector2::new(x, character.baseline_shift as f32 * scale.y),
                    Vector2::zeros(),
                    Vector2::zeros(),
                ],
                color: Vector3::new(self.color.r, self.color.g, self.color.b),
            });

            x += (character.size.x as f32 + font.desc.tracking) * scale.x;
            n += 1;
        }

        let line_size = Vector2::new(x, 0.0);
        for i in ctx.sprites.len() - n..ctx.sprites.len() {
            let [size, offset, ..] = ctx.sprites[i].points;
            let pos = self
                .anchor
                .offset(self.pos + offset, line_size)
                .map(|x| x.round());

            ctx.sprites[i].points = [
                pos,
                pos + Vector2::new(0.0, size.y),
                pos + size,
                pos + Vector2::new(size.x, 0.0),
            ];
        }
    }
}
