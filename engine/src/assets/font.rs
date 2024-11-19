use std::collections::HashMap;

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
