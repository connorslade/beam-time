use engine::assets::{
    constructor::AssetConstructor, font::FontDescriptor, AudioRef, FontRef, SpriteRef,
};
use image::RgbaImage;

use crate::{include_atlas, util::include_asset};

// fonts
pub const UNDEAD_FONT: FontRef = FontRef::new("undead_font");
pub const ALAGARD_FONT: FontRef = FontRef::new("alagard_font");

// audio
pub const INTRO_MUSIC: AudioRef = AudioRef::new("intro_music");
pub const BUTTON_HOVER: AudioRef = AudioRef::new("button_hover");

// interface
pub const TITLE: SpriteRef = SpriteRef::new("title");
pub const COPYRIGHT: SpriteRef = SpriteRef::new("copyright");
pub const BACK_BUTTON: SpriteRef = SpriteRef::new("back_button");
pub const START_BUTTON: SpriteRef = SpriteRef::new("start_button");
pub const OPTIONS_BUTTON: SpriteRef = SpriteRef::new("options_button");
pub const ABOUT_BUTTON: SpriteRef = SpriteRef::new("about_button");

// tiles
pub const MIRROR_A_TILE: SpriteRef = SpriteRef::new("mirror_a_tile");
pub const MIRROR_B_TILE: SpriteRef = SpriteRef::new("mirror_b_tile");
pub const WALL_TILE: SpriteRef = SpriteRef::new("wall_tile");
pub const GALVO_TILE: SpriteRef = SpriteRef::new("galvo_tile");
pub const EYE_TILE: SpriteRef = SpriteRef::new("eye_tile");

pub fn init(assets: &mut AssetConstructor) {
    // assets.register_audio(INTRO_MUSIC, include_asset!("sounds/intro-music.mp3"));
    // assets.register_audio(BUTTON_HOVER, include_asset!("sounds/button-hover.mp3"));

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

fn load_font(assets: &mut AssetConstructor, asset: FontRef, atlas: RgbaImage, descriptor: &[u8]) {
    let font = assets.register_atlas(atlas);
    let descriptor = ron::de::from_bytes::<FontDescriptor>(descriptor).unwrap();
    assets.register_font(font, asset, descriptor);
}
