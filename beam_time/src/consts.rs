use std::f32::consts::PI;

use engine::{assets::SpriteRef, color::Rgb};

use crate::assets::{EYE_TILE, GALVO_TILE, MIRROR_TILE, SPLITTER_TILE, WALL_TILE};

pub const DEFAULT_SIZE: (u32, u32) = (800, 600);
pub const BACKGROUND_COLOR: Rgb<f32> = Rgb::new(0.294, 0.184, 0.224);
pub const LIGHT_BACKGROUND: Rgb<f32> = Rgb::new(0.341, 0.216, 0.259);
pub const FOREGROUND_COLOR: Rgb<f32> = Rgb::new(0.859, 0.89, 0.839);
pub const ACCENT_COLOR: Rgb<f32> = Rgb::new(0.812, 0.306, 0.306);

pub const TILES: [SpriteRef; 5] = [MIRROR_TILE, SPLITTER_TILE, WALL_TILE, GALVO_TILE, EYE_TILE];
pub const TILE_ROTATION: [f32; 5] = [PI / 2.0, 2.0 * PI, 0.0, 0.0, 0.0];
