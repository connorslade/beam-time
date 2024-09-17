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

pub fn init(assets: &mut AssetConstructor) {
    let title = assets.register_atlas(include_atlas!("title.png"));
    assets.register_sprite(title, TITLE, (0, 0), (81, 20));
    assets.register_sprite(title, COPYRIGHT, (0, 20), (28, 8));

    let tiles = assets.register_atlas(include_atlas!("tilemap.png"));
    assets.register_sprite(tiles, EYE_TILE, (64, 16), (16, 16));
    assets.register_sprite(tiles, BALL, (80, 16), (8, 8));
    assets.register_sprite(tiles, PADDLE, (96, 16), (3, 16));

    let font = assets.register_atlas(include_atlas!("font.png"));
    let descriptor =
        ron::de::from_str::<FontDescriptor>(include_str!("../assets/font.ron")).unwrap();
    assets.register_font(font, DEFAULT_FONT, descriptor);
}
