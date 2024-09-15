use std::{collections::HashMap, str::Chars};

use nalgebra::Vector2;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct FontDescriptor {
    pub characters: HashMap<char, Character>,
    pub unknown: Character,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Character {
    pub uv: Vector2<u32>,
    pub size: Vector2<u32>,
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

impl<'a> Iterator for FontCharacterIterator<'a> {
    type Item = Character;

    fn next(&mut self) -> Option<Self::Item> {
        let next_char = self.string.next()?;
        let character = self.font_descriptor.characters.get(&next_char).copied();
        Some(character.unwrap_or(self.font_descriptor.unknown))
    }
}
