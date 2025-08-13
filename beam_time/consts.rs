use std::time::Duration;

use engine::{exports::winit::keyboard::KeyCode, memory::MemoryKey, memory_key};
use once_cell::sync::Lazy;
use url::Url;

#[cfg(feature = "steam")]
pub const STEAM_ID: u32 = 3385920;
pub const MAX_HISTORY: usize = 100;
pub const AUTOSAVE_INTERVAL: Duration = Duration::from_secs(60 * 5);

pub const AUTHOR_HOMEPAGE: &str = "https://connorcode.com";
pub const GAME_HOMEPAGE: &str =
    "https://store.steampowered.com/app/3385920/Beam_Time?utm_source=in_game";
pub static LEADERBOARD_SERVER: Lazy<Url> =
    Lazy::new(|| Url::parse("https://beamtime.connorcode.com/api/").unwrap());

pub mod color {
    use engine::color::Rgb;

    pub const BACKGROUND: Rgb<f32> = Rgb::repeat(0.235);
    pub const ACCENT: Rgb<f32> = Rgb::hex(0xE27285);
    pub const SELECTION: Rgb<f32> = Rgb::hex(0xE27285);
    pub const ERROR: Rgb<f32> = Rgb::hex(0xE43636);
    pub const MODAL: Rgb<f32> = Rgb::hex(0xA6A6A6);
    pub const MODAL_BORDER: Rgb<f32> = Rgb::hex(0x757575);
}

pub mod layer {
    pub const TILE_HOLDING: i16 = 8;
    pub const TILE_HOLDING_BACKGROUND: i16 = 7;

    // Jumps to 6 to allow for one layer between UI elements and the pause modal
    // to render tile labels
    pub const UI_OVERLAY: i16 = 6;
    pub const UI_ELEMENT: i16 = 4;
    pub const UI_BACKGROUND: i16 = 3;

    pub const OVERLAY: i16 = 2;
    pub const LASER: i16 = 1;
    pub const UNDER_LASER: i16 = -1;
    pub const TILE_BACKGROUND_OVERLAY: i16 = -2;
    pub const TILE_BACKGROUND: i16 = -3;
}

/// All relative to data_dir.
pub mod paths {
    pub const CAMPAIGN: &str = "campaign";
    pub const SANDBOX: &str = "sandbox";

    pub const CONFIG: &str = "config.toml";
    pub const SOLVED: &str = "solved.bin";
}

pub const WATERFALL: MemoryKey = memory_key!();
pub const CTRL: KeyCode =
    [KeyCode::ControlLeft, KeyCode::SuperLeft][cfg!(target_os = "macos") as usize];
pub const KEYBINDS: &[(&str, &str)] = &[
    ("T", "Runs test cases"),
    ("F", "Starts/stops simulation"),
    ("R", "Rotates the held or hovered tile"),
    ("Q", "Copy hovered tile"),
    ("E", "Toggle the held or hovered emitter"),
];
