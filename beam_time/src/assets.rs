use engine::assets::{asset, constructor::AssetConstructor, font::FontDescriptor, AssetRef};

use crate::{include_atlas, util::include_assets};

// fonts
pub const DEFAULT_FONT: AssetRef = asset("default_font");

// interface
pub const TITLE: AssetRef = asset("title");
pub const COPYRIGHT: AssetRef = asset("copyright");
pub const BACK_BUTTON: AssetRef = asset("back_button");
pub const START_BUTTON: AssetRef = asset("start_button");
pub const OPTIONS_BUTTON: AssetRef = asset("options_button");
pub const ABOUT_BUTTON: AssetRef = asset("about_button");

// tiles
pub const MIRROR_A_TILE: AssetRef = asset("mirror_a_tile");
pub const MIRROR_B_TILE: AssetRef = asset("mirror_b_tile");
pub const WALL_TILE: AssetRef = asset("wall_tile");
pub const GALVO_TILE: AssetRef = asset("galvo_tile");
pub const EYE_TILE: AssetRef = asset("eye_tile");

// temporary
pub const BALL: AssetRef = asset("ball");
pub const PADDLE: AssetRef = asset("paddle");

pub fn init(assets: &mut AssetConstructor) {
    let interface = assets.register_atlas(include_atlas!("interface.png"));
    assets.register_sprite(interface, TITLE, (0, 0), (81, 20));
    assets.register_sprite(interface, COPYRIGHT, (0, 20), (30, 8));
    assets.register_sprite(interface, BACK_BUTTON, (58, 32), (32, 14));
    assets.register_sprite(interface, START_BUTTON, (0, 32), (57, 14));
    assets.register_sprite(interface, OPTIONS_BUTTON, (0, 48), (39, 14));
    assets.register_sprite(interface, ABOUT_BUTTON, (40, 48), (31, 14));

    let tiles = assets.register_atlas(include_atlas!("tilemap.png"));
    assets.register_sprite(tiles, MIRROR_A_TILE, (0, 0), (16, 16));
    assets.register_sprite(tiles, MIRROR_B_TILE, (16, 0), (16, 16));
    assets.register_sprite(tiles, WALL_TILE, (80, 0), (16, 16));
    assets.register_sprite(tiles, GALVO_TILE, (112, 0), (16, 16));
    assets.register_sprite(tiles, EYE_TILE, (64, 16), (16, 16));
    assets.register_sprite(tiles, BALL, (80, 16), (8, 8));
    assets.register_sprite(tiles, PADDLE, (96, 16), (3, 16));

    let font = assets.register_atlas(include_atlas!("fonts/undead-pixel-11.png"));
    let descriptor =
        ron::de::from_bytes::<FontDescriptor>(include_assets!("fonts/undead-pixel-11.ron"))
            .unwrap();
    assets.register_font(font, DEFAULT_FONT, descriptor);
}
