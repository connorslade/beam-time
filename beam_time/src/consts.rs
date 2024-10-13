use engine::{assets::SpriteRef, color::Rgb};

use crate::assets::{
    BEAM_HALF_DOWN, BEAM_HALF_LEFT, BEAM_HALF_RIGHT, BEAM_HALF_UP, TILE_DELAY, TILE_DETECTOR,
    TILE_EMITTER_DOWN, TILE_EMITTER_LEFT, TILE_EMITTER_RIGHT, TILE_EMITTER_UP, TILE_GALVO_DOWN,
    TILE_GALVO_LEFT, TILE_GALVO_RIGHT, TILE_GALVO_UP, TILE_MIRROR_A, TILE_MIRROR_B,
    TILE_SPLITTER_A, TILE_SPLITTER_B, TILE_WALL,
};

pub const DEFAULT_SIZE: (u32, u32) = (800, 600);
pub const CONFIG_FILE: &str = "config.toml";

pub const BACKGROUND_COLOR: Rgb<f32> = Rgb::new(0.235, 0.235, 0.235);
pub const FOREGROUND_COLOR: Rgb<f32> = Rgb::new(0.859, 0.89, 0.839);
pub const ACCENT_COLOR: Rgb<f32> = Rgb::new(0.812, 0.306, 0.306);

pub mod layer {
    pub const TILE_HOLDING: i16 = 2;
    pub const LASER: i16 = 1;
    pub const UNDER_LASER: i16 = -1;
    pub const TILE_BACKGROUND_OVERLAY: i16 = -2;
    pub const TILE_BACKGROUND: i16 = -3;
}

pub const TILES: [SpriteRef; 9] = [
    TILE_MIRROR_A,
    TILE_MIRROR_B,
    TILE_SPLITTER_A,
    TILE_SPLITTER_B,
    TILE_WALL,
    TILE_GALVO_UP,
    TILE_EMITTER_UP,
    TILE_DETECTOR,
    TILE_DELAY,
];

pub const GALVO: [SpriteRef; 4] = [
    TILE_GALVO_UP,
    TILE_GALVO_RIGHT,
    TILE_GALVO_DOWN,
    TILE_GALVO_LEFT,
];

pub const EMITTER: [SpriteRef; 4] = [
    TILE_EMITTER_UP,
    TILE_EMITTER_RIGHT,
    TILE_EMITTER_DOWN,
    TILE_EMITTER_LEFT,
];

pub const SPLITTER: [SpriteRef; 2] = [TILE_SPLITTER_A, TILE_SPLITTER_B];
pub const MIRROR: [SpriteRef; 2] = [TILE_MIRROR_A, TILE_MIRROR_B];

pub const HALF_BEAM: [SpriteRef; 4] = [
    BEAM_HALF_UP,
    BEAM_HALF_RIGHT,
    BEAM_HALF_DOWN,
    BEAM_HALF_LEFT,
];
