use std::time::Duration;

use engine::{assets::SpriteRef, color::Rgb, memory::MemoryKey, memory_key};
use indoc::indoc;
use once_cell::sync::Lazy;
use url::Url;

use crate::assets::{
    BEAM_HALF_DOWN, BEAM_HALF_LEFT, BEAM_HALF_RIGHT, BEAM_HALF_UP, TILE_DELAY, TILE_DETECTOR,
    TILE_EMITTER_DOWN, TILE_EMITTER_LEFT, TILE_EMITTER_RIGHT, TILE_EMITTER_UP, TILE_GALVO_DOWN,
    TILE_GALVO_LEFT, TILE_GALVO_RIGHT, TILE_GALVO_UP, TILE_MIRROR_A, TILE_MIRROR_B,
    TILE_SPLITTER_A, TILE_SPLITTER_B, TILE_WALL,
};

#[cfg(feature = "steam")]
pub const STEAM_ID: u32 = 3385920;
pub const CONFIG_FILE: &str = "config.toml";
pub const MAX_HISTORY: usize = 100;
pub const AUTOSAVE_INTERVAL: Duration = Duration::from_secs(60 * 5);

pub const AUTHOR_HOMEPAGE: &str = "https://connorcode.com";
pub const GAME_HOMEPAGE: &str = "https://store.steampowered.com/app/3385920/Beam_Time";
pub static LEADERBOARD_SERVER: Lazy<Url> =
    Lazy::new(|| Url::parse("http://localhost:8080/api/").unwrap());

pub const BACKGROUND_COLOR: Rgb<f32> = Rgb::repeat(0.235);
pub const ACCENT_COLOR: Rgb<f32> = Rgb::hex(0xE27285);
pub const SELECTION_COLOR: Rgb<f32> = Rgb::hex(0xE27285);
pub const ERROR_COLOR: Rgb<f32> = Rgb::hex(0xE43636);
pub const MODAL_COLOR: Rgb<f32> = Rgb::hex(0xA6A6A6);
pub const MODAL_BORDER_COLOR: Rgb<f32> = Rgb::hex(0x757575);

pub const KEYBINDS: &[(&str, &str)] = &[
    ("T", "Runs test cases"),
    ("P", "Starts simulation"),
    ("R", "Rotates current tile"),
    ("Q", "Copy hovered tile"),
    ("E", "Toggle hovered emitter"),
];

pub const DESCRIPTION: &str = indoc! {"
    Beam time is a logic puzzle game where you redirect and split laser beams to create digital circuits.

    Thank you to everyone that pushed me to actually finish this project ♥. \
    Special thanks to Brandon Li (aspiringLich on GitHub) for creating the tile graphics, you do not want to see what the game looked like before.

    This is not an open source project, however the source code for the custom engine, leaderboard server, and the game itself is available on Github at @connorslade/beam-time.

    Assets Used:
      • Alagard, Font by Hewett Tsoi
      • Undead Pixel Light, Font by Not Jam
      • Universal UI/Menu Soundpack, by Cyrex Studios
"};

pub mod layer {
    pub const TILE_HOLDING: i16 = 7;
    pub const TILE_HOLDING_BACKGROUND: i16 = 6;

    pub const UI_OVERLAY: i16 = 5;
    pub const UI_ELEMENT: i16 = 4;
    pub const UI_BACKGROUND: i16 = 3;

    pub const OVERLAY: i16 = 2;
    pub const LASER: i16 = 1;
    pub const UNDER_LASER: i16 = -1;
    pub const TILE_BACKGROUND_OVERLAY: i16 = -2;
    pub const TILE_BACKGROUND: i16 = -3;
}

pub const WATERFALL: MemoryKey = memory_key!();
pub const TILES: &[&[SpriteRef]] = &[
    &MIRROR,
    &SPLITTER,
    &[TILE_WALL],
    &GALVO,
    &EMITTER,
    &[TILE_DETECTOR],
    &[TILE_DELAY],
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
