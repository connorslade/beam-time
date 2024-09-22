use std::{collections::HashMap, str::Chars};

use nalgebra::Vector2;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct FontDescriptor {
    pub characters: HashMap<char, Character>,
    pub unknown: Character,
    pub height: f32,
    pub leading: f32,
    pub space_width: f32,
    pub tracking: f32,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Character {
    pub uv: Vector2<u32>,
    pub size: Vector2<u32>,
    #[serde(default)]
    pub baseline_shift: i32,
}

impl FontDescriptor {
    pub(crate) fn process_string<'a>(&'a self, string: &'a str) -> FontCharacterIterator<'a> {
        FontCharacterIterator {
            font_descriptor: self,
            string: string.chars(),
        }
    }
}

pub(crate) struct FontCharacterIterator<'a> {
    font_descriptor: &'a FontDescriptor,
    string: Chars<'a>,
}

pub(crate) enum FontChar {
    Char(Character),
    Space,
    Newline,
}

impl<'a> Iterator for FontCharacterIterator<'a> {
    type Item = FontChar;

    fn next(&mut self) -> Option<Self::Item> {
        let next_char = self.string.next()?;

        if next_char == '\n' {
            return Some(FontChar::Newline);
        }

        if next_char.is_whitespace() {
            return Some(FontChar::Space);
        }

        let character = self.font_descriptor.characters.get(&next_char).copied();
        let character = FontChar::Char(character.unwrap_or(self.font_descriptor.unknown));

        Some(character)
    }
}
