use engine::assets::{asset, constructor::AssetConstructor, font::FontDescriptor, AssetRef};

use crate::include_atlas;

// fonts
pub const DEFAULT_FONT: AssetRef = asset("default_font");

// sprites
pub const TITLE: AssetRef = asset("title");
pub const COPYRIGHT: AssetRef = asset("copyright");

// tiles
pub const EYE_TILE: AssetRef = asset("eye_tile");
pub const BALL: AssetRef = asset("ball");
pub const PADDLE: AssetRef = asset("paddle");

pub const START_BUTTON: AssetRef = asset("start_button");
pub const OPTIONS_BUTTON: AssetRef = asset("options_button");
pub const ABOUT_BUTTON: AssetRef = asset("about_button");

// button elements
pub const BUTTON_LEFT_CAP: AssetRef = asset("button_left_cap");
pub const BUTTON_RIGHT_CAP: AssetRef = asset("button_right_cap");
pub const BUTTON_MIDDLE: AssetRef = asset("button_middle");

pub fn init(assets: &mut AssetConstructor) {
    let interface = assets.register_atlas(include_atlas!("interface.png"));
    assets.register_sprite(interface, TITLE, (0, 0), (81, 20));
    assets.register_sprite(interface, COPYRIGHT, (0, 20), (28, 8));

    assets.register_sprite(interface, BUTTON_LEFT_CAP, (30, 21), (4, 14));
    assets.register_sprite(interface, BUTTON_RIGHT_CAP, (52, 21), (4, 14));
    assets.register_sprite(interface, BUTTON_MIDDLE, (35, 21), (16, 14));

    let tiles = assets.register_atlas(include_atlas!("tilemap.png"));
    assets.register_sprite(tiles, EYE_TILE, (64, 16), (16, 16));
    assets.register_sprite(tiles, BALL, (80, 16), (8, 8));
    assets.register_sprite(tiles, PADDLE, (96, 16), (3, 16));

    assets.register_sprite(tiles, START_BUTTON, (0, 32), (59, 14));
    assets.register_sprite(tiles, OPTIONS_BUTTON, (0, 48), (41, 14));
    assets.register_sprite(tiles, ABOUT_BUTTON, (0, 64), (32, 14));

    let font = assets.register_atlas(include_atlas!("font.png"));
    let descriptor =
        ron::de::from_str::<FontDescriptor>(include_str!("../assets/font.ron")).unwrap();
    assets.register_font(font, DEFAULT_FONT, descriptor);
}
