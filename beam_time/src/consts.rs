use engine::{assets::SpriteRef, color::Rgb};

use crate::assets::{
    TILE_DETECTOR, TILE_EMITTER_UP, TILE_GALVO_UP, TILE_MIRROR_A, TILE_MIRROR_B, TILE_SPLITTER_A,
    TILE_SPLITTER_B, TILE_WALL,
};

pub const DEFAULT_SIZE: (u32, u32) = (800, 600);
pub const BACKGROUND_COLOR: Rgb<f32> = Rgb::new(0.294, 0.184, 0.224);
pub const LIGHT_BACKGROUND: Rgb<f32> = Rgb::new(0.341, 0.216, 0.259);
pub const FOREGROUND_COLOR: Rgb<f32> = Rgb::new(0.859, 0.89, 0.839);
pub const ACCENT_COLOR: Rgb<f32> = Rgb::new(0.812, 0.306, 0.306);

pub const TILES: [SpriteRef; 8] = [
    TILE_MIRROR_A,
    TILE_MIRROR_B,
    TILE_SPLITTER_A,
    TILE_SPLITTER_B,
    TILE_WALL,
    TILE_GALVO_UP,
    TILE_EMITTER_UP,
    TILE_DETECTOR,
];
