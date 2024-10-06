use engine::{
    assets::{constructor::AssetConstructor, font::FontDescriptor, AudioRef, FontRef, SpriteRef},
    define_refs,
};
use image::RgbaImage;

use crate::{include_atlas, util::include_asset};

define_refs! {
    FontRef => {
        UNDEAD_FONT,
        ALAGARD_FONT
    },
    AudioRef => {
        INTRO_MUSIC,
        BUTTON_HOVER,
        BUTTON_SUCCESS,
        BUTTON_BACK
    },
    SpriteRef => {
        TITLE,
        COPYRIGHT,

        BACK_BUTTON,
        START_BUTTON,
        OPTIONS_BUTTON,
        ABOUT_BUTTON,

        EMPTY_TILE,
        EMPTY_TILE_TOP,
        EMPTY_TILE_RIGHT,
        EMPTY_TILE_TOP_RIGHT,

        MIRROR_TILE,
        SPLITTER_TILE,
        WALL_TILE,
        GALVO_TILE,
        EMITTER_TILE,
        EYE_TILE,

        BEAM,
        CROSS_BEAM,
        MIRROR_BEAM,
        SPLITTER_BEAM
    }
}

pub fn init(assets: &mut AssetConstructor) {
    assets.register_audio(INTRO_MUSIC, include_asset!("sounds/intro-music.mp3"));
    assets.register_audio(BUTTON_HOVER, include_asset!("sounds/button-hover.mp3"));
    assets.register_audio(BUTTON_SUCCESS, include_asset!("sounds/button-success.mp3"));
    assets.register_audio(BUTTON_BACK, include_asset!("sounds/button-back.mp3"));

    let interface = assets.register_atlas(include_atlas!("interface.png"));
    assets.register_sprite(interface, TITLE, (0, 0), (81, 20));
    assets.register_sprite(interface, COPYRIGHT, (0, 20), (30, 8));
    assets.register_sprite(interface, BACK_BUTTON, (58, 32), (26, 14));
    assets.register_sprite(interface, START_BUTTON, (0, 32), (57, 14));
    assets.register_sprite(interface, OPTIONS_BUTTON, (0, 48), (39, 14));
    assets.register_sprite(interface, ABOUT_BUTTON, (40, 48), (31, 14));

    let tiles = assets.register_atlas(include_atlas!("tilemap.png"));
    assets.register_sprite(tiles, EMPTY_TILE, (16, 16), (16, 16));
    assets.register_sprite(tiles, EMPTY_TILE_TOP, (32, 16), (16, 16));
    assets.register_sprite(tiles, EMPTY_TILE_RIGHT, (48, 16), (16, 16));
    assets.register_sprite(tiles, EMPTY_TILE_TOP_RIGHT, (0, 16), (16, 16));

    assets.register_sprite(tiles, MIRROR_TILE, (0, 0), (16, 16));
    assets.register_sprite(tiles, SPLITTER_TILE, (16, 0), (16, 16));
    assets.register_sprite(tiles, WALL_TILE, (64, 0), (16, 16));
    assets.register_sprite(tiles, GALVO_TILE, (80, 0), (16, 16));
    assets.register_sprite(tiles, EMITTER_TILE, (32, 0), (16, 16));
    assets.register_sprite(tiles, EYE_TILE, (48, 0), (16, 16));

    assets.register_sprite(tiles, BEAM, (96, 0), (16, 16));
    assets.register_sprite(tiles, CROSS_BEAM, (80, 16), (16, 16));
    assets.register_sprite(tiles, MIRROR_BEAM, (96, 16), (16, 16));
    assets.register_sprite(tiles, SPLITTER_BEAM, (64, 16), (16, 16));

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
