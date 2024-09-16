use nalgebra::{Vector2, Vector3};

use crate::{
    assets::{font::FontChar, AssetRef},
    graphics_context::{Anchor, Drawable, GraphicsContext},
    render::sprite::GpuSprite,
};

pub struct Text<'a> {
    font: AssetRef,
    text: &'a str,

    pos: Vector2<f32>,
    anchor: Anchor,
    scale: Vector2<f32>,
    color: Vector3<f32>,
}

impl<'a> Text<'a> {
    pub fn new(font: AssetRef, text: &'a str) -> Self {
        Self {
            font,
            text,

            pos: Vector2::repeat(0.0),
            anchor: Anchor::BottomLeft,
            scale: Vector2::repeat(1.0),
            color: Vector3::repeat(1.0),
        }
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

    pub fn color(mut self, color: Vector3<f32>) -> Self {
        self.color = color;
        self
    }

    pub fn build(self) -> Text<'a> {
        Text {
            font: self.font,
            text: self.text,
            pos: self.pos,
            anchor: self.anchor,
            scale: self.scale,
            color: self.color,
        }
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
                pos: (
                    size,
                    Vector2::new(x, character.baseline_shift as f32 * scale.y),
                ),
                color: self.color,
            });

            x += (character.size.x as f32 + font.desc.tracking) * scale.x;
            n += 1;
        }

        let line_size = Vector2::new(x, 0.0);
        for i in ctx.sprites.len() - n..ctx.sprites.len() {
            let (size, offset) = ctx.sprites[i].pos;
            let pos = self.anchor.offset(self.pos + offset, line_size);
            ctx.sprites[i].pos = (pos, pos + size);
        }
    }
}
