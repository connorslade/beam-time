use engine::assets::{asset, constructor::AssetConstructor, font::FontDescriptor, AssetRef};
use image::RgbaImage;

use crate::{include_atlas, util::include_asset};

// fonts
pub const UNDEAD_FONT: AssetRef = asset("undead_font");
pub const ALAGARD_FONT: AssetRef = asset("alagard_font");

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
    assets.register_sprite(interface, BACK_BUTTON, (58, 32), (26, 14));
    assets.register_sprite(interface, START_BUTTON, (0, 32), (57, 14));
    assets.register_sprite(interface, OPTIONS_BUTTON, (0, 48), (39, 14));
    assets.register_sprite(interface, ABOUT_BUTTON, (40, 48), (31, 14));

    let tiles = assets.register_atlas(include_atlas!("tilemap.png"));
    assets.register_sprite(tiles, MIRROR_A_TILE, (0, 0), (16, 16));
    assets.register_sprite(tiles, MIRROR_B_TILE, (16, 0), (16, 16));
    assets.register_sprite(tiles, WALL_TILE, (80, 0), (16, 16));
    assets.register_sprite(tiles, GALVO_TILE, (112, 0), (16, 16));
    assets.register_sprite(tiles, EYE_TILE, (64, 16), (16, 16));
    assets.register_sprite(tiles, BALL, (81, 17), (8, 8));
    assets.register_sprite(tiles, PADDLE, (96, 16), (3, 16));

    load_font(
        assets,
        UNDEAD_FONT,
        include_atlas!("fonts/undead-pixel-11.png"),
        include_asset!("fonts/undead-pixel-11.ron"),
    );

    load_font(
        assets,
        ALAGARD_FONT,
        include_atlas!("fonts/alagard.png"),
        include_asset!("fonts/alagard.ron"),
    )
}

fn load_font(assets: &mut AssetConstructor, asset: AssetRef, atlas: RgbaImage, descriptor: &[u8]) {
    let font = assets.register_atlas(atlas);
    let descriptor = ron::de::from_bytes::<FontDescriptor>(descriptor).unwrap();
    assets.register_font(font, asset, descriptor);
}
