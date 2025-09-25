use std::time::Duration;

use engine::{memory::MemoryKey, memory_key};
use once_cell::sync::Lazy;
use url::Url;

pub const MAX_HISTORY: usize = 100;
pub const AUTOSAVE_INTERVAL: Duration = Duration::from_secs(60 * 5);
pub const WATERFALL: MemoryKey = memory_key!();

pub const AUTHOR_HOMEPAGE: &str = "https://connorcode.com";
pub const GAME_HOMEPAGE: &str =
    "https://store.steampowered.com/app/3385920/Beam_Time?utm_source=in_game";
pub static LEADERBOARD_SERVER: Lazy<Url> =
    Lazy::new(|| Url::parse("https://beamtime.connorcode.com/api/").unwrap());

#[cfg(feature = "steam")]
pub const STEAM_ID: u32 = 3385920;
#[cfg(feature = "discord")]
pub const DISCORD_ID: u64 = 1420274195216207933;

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

pub mod spacing {
    pub const MARGIN: f32 = 16.0;
    pub const PADDING: f32 = 10.0;
}

/// All relative to data_dir.
pub mod paths {
    pub const CAMPAIGN: &str = "campaign";
    pub const SANDBOX: &str = "sandbox";

    pub const CONFIG: &str = "config.toml";
    pub const SOLVED: &str = "solved.bin";
}

pub mod keybind {
    use engine::exports::winit::keyboard::KeyCode;

    // modifiers
    pub const CTRL: KeyCode =
        [KeyCode::ControlLeft, KeyCode::SuperLeft][cfg!(target_os = "macos") as usize];
    pub const SHIFT: KeyCode = KeyCode::ShiftLeft;
    pub const ALT: KeyCode = KeyCode::AltLeft;

    // navigation
    pub const BACK: KeyCode = KeyCode::Escape;
    pub const CONTINUE: KeyCode = KeyCode::Enter;
    pub const OVERWRITE: KeyCode = KeyCode::AltLeft;

    // selections
    pub const UNDO: KeyCode = KeyCode::KeyZ;
    pub const COPY: KeyCode = KeyCode::KeyC;
    pub const CUT: KeyCode = KeyCode::KeyX;
    pub const PASTE: KeyCode = KeyCode::KeyV;

    pub const DESELECT: KeyCode = KeyCode::KeyU;
    pub const DELETE: KeyCode = KeyCode::Delete;

    // tile operations
    pub const ROTATE: KeyCode = KeyCode::KeyR;
    pub const TOGGLE: KeyCode = KeyCode::KeyE;
    pub const FLIP_V: KeyCode = KeyCode::KeyV;
    pub const FLIP_H: KeyCode = KeyCode::KeyH;
    pub const PICK: KeyCode = KeyCode::KeyQ;

    // game stuff
    pub const PLAY: KeyCode = KeyCode::KeyF;
    pub const TEST: KeyCode = KeyCode::KeyT;
    pub const STEP: KeyCode = KeyCode::Space;

    pub const NOTE: KeyCode = KeyCode::KeyN;
    pub const SPEED_UP: KeyCode = KeyCode::Equal;
    pub const SPEED_DOWN: KeyCode = KeyCode::Minus;
    pub const SPEED_RESET: KeyCode = KeyCode::Digit0;

    // pancam movement
    pub const UP: KeyCode = KeyCode::KeyW;
    pub const DOWN: KeyCode = KeyCode::KeyS;
    pub const LEFT: KeyCode = KeyCode::KeyA;
    pub const RIGHT: KeyCode = KeyCode::KeyD;
}
