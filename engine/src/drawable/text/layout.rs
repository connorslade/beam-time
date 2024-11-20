use nalgebra::Vector2;

use crate::assets::font::{Character, FontDescriptor};

pub struct TextLayout {
    pub chars: Vec<(Character, Vector2<f32>)>,
    pub size: Vector2<f32>,
}

impl TextLayout {
    pub fn generate(
        font: &FontDescriptor,
        max_width: f32,
        scale: Vector2<f32>,
        text: &str,
    ) -> Self {
        let mut chars = Vec::new();
        let mut pos = Vector2::zeros();
        let mut width = 0_f32;
        let mut last_space = 0;

        let line_height = (font.height + font.leading) * scale.y;

        for chr in text.chars() {
            match chr {
                '\r' => {}
                '\n' => {
                    pos.x = 0.0;
                    pos.y -= line_height;
                }
                ' ' => {
                    pos.x += font.space_width * scale.x;
                    last_space = chars.len();
                }
                c => {
                    let character = font.characters.get(&c).copied();
                    let character = character.unwrap_or(font.unknown);
                    chars.push((character, pos));

                    let char_width = character.size.x as f32 * scale.x;
                    pos.x += char_width + font.tracking * scale.x;
                }
            }

            // todo: fallback for words longer than max_width
            if pos.x > max_width {
                pos.x = 0.0;
                pos.y -= line_height;

                for (character, chr_pos) in chars[last_space..].iter_mut() {
                    let char_width = character.size.x as f32 * scale.x;
                    *chr_pos = pos;
                    pos.x += char_width + font.tracking * scale.x;
                }
            }

            width = width.max(pos.x - font.tracking * scale.x);
        }

        chars.iter_mut().for_each(|(_, x)| x.y -= pos.y);
        let size = Vector2::new(width, -pos.y + font.height * scale.y);
        Self { chars, size }
    }
}
