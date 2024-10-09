use engine::{
    assets::{constructor::AssetConstructor, font::FontDescriptor, AudioRef, FontRef, SpriteRef},
    define_refs,
    drawable::sprite::Sprite,
    exports::nalgebra::Vector2,
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

        EMPTY_TILE_A,
        EMPTY_TILE_B,
        PERMANENT_TILE,

        TILE_WALL,
        TILE_DETECTOR,
        TILE_MIRROR_A,
        TILE_MIRROR_B,
        TILE_SPLITTER_A,
        TILE_SPLITTER_B,
        TILE_GALVO_UP,
        TILE_GALVO_RIGHT,
        TILE_GALVO_DOWN,
        TILE_GALVO_LEFT,
        TILE_EMITTER_UP,
        TILE_EMITTER_RIGHT,
        TILE_EMITTER_DOWN,
        TILE_EMITTER_LEFT,

        BEAM_FULL_HORIZONTAL,
        BEAM_FULL_VERTICAL,
        BEAM_REFLECT_UP_LEFT,
        BEAM_REFLECT_DOWN_LEFT,
        BEAM_REFLECT_UP_RIGHT,
        BEAM_REFLECT_DOWN_RIGHT,
        BEAM_SPLIT_UP,
        BEAM_SPLIT_RIGHT,
        BEAM_SPLIT_DOWN,
        BEAM_SPLIT_LEFT,
        BEAM_HALF_UP,
        BEAM_HALF_RIGHT,
        BEAM_HALF_DOWN,
        BEAM_HALF_LEFT
    }
}

pub fn animated_sprite(texture: SpriteRef, active: bool, frame: u8) -> Sprite {
    let offset = if active { frame + 1 } else { 0 } * 16;
    Sprite::new(texture).uv_offset(Vector2::new(offset as u32, 0))
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
    assets.register_sprite(tiles, EMPTY_TILE_A, (0, 256), (16, 16));
    assets.register_sprite(tiles, EMPTY_TILE_B, (16, 256), (16, 16));
    assets.register_sprite(tiles, PERMANENT_TILE, (0, 240), (16, 16));

    assets.register_sprite(tiles, TILE_WALL, (0, 176), (16, 16));
    assets.register_sprite(tiles, TILE_DETECTOR, (0, 160), (16, 16));
    assets.register_sprite(tiles, TILE_MIRROR_A, (0, 64), (16, 16));
    assets.register_sprite(tiles, TILE_MIRROR_B, (0, 80), (16, 16));
    assets.register_sprite(tiles, TILE_SPLITTER_A, (0, 128), (16, 16));
    assets.register_sprite(tiles, TILE_SPLITTER_B, (0, 144), (16, 16));
    assets.register_sprite(tiles, TILE_GALVO_UP, (64, 192), (16, 16));
    assets.register_sprite(tiles, TILE_GALVO_RIGHT, (0, 192), (16, 16));
    assets.register_sprite(tiles, TILE_GALVO_DOWN, (64, 208), (16, 16));
    assets.register_sprite(tiles, TILE_GALVO_LEFT, (0, 208), (16, 16));
    assets.register_sprite(tiles, TILE_EMITTER_UP, (64, 96), (16, 16));
    assets.register_sprite(tiles, TILE_EMITTER_DOWN, (64, 112), (16, 16));
    assets.register_sprite(tiles, TILE_EMITTER_LEFT, (0, 112), (16, 16));
    assets.register_sprite(tiles, TILE_EMITTER_RIGHT, (0, 96), (16, 16));

    assets.register_sprite(tiles, BEAM_FULL_HORIZONTAL, (16, 32), (16, 16));
    assets.register_sprite(tiles, BEAM_FULL_VERTICAL, (16, 48), (16, 16));
    assets.register_sprite(tiles, BEAM_REFLECT_UP_LEFT, (80, 32), (16, 16));
    assets.register_sprite(tiles, BEAM_REFLECT_DOWN_LEFT, (80, 48), (16, 16));
    assets.register_sprite(tiles, BEAM_REFLECT_UP_RIGHT, (80, 80), (16, 16));
    assets.register_sprite(tiles, BEAM_REFLECT_DOWN_RIGHT, (80, 64), (16, 16));
    assets.register_sprite(tiles, BEAM_SPLIT_UP, (80, 128), (16, 16));
    assets.register_sprite(tiles, BEAM_SPLIT_RIGHT, (80, 144), (16, 16));
    assets.register_sprite(tiles, BEAM_SPLIT_DOWN, (80, 160), (16, 16));
    assets.register_sprite(tiles, BEAM_SPLIT_LEFT, (80, 176), (16, 16));
    assets.register_sprite(tiles, BEAM_HALF_UP, (80, 0), (16, 16));
    assets.register_sprite(tiles, BEAM_HALF_RIGHT, (16, 0), (16, 16));
    assets.register_sprite(tiles, BEAM_HALF_DOWN, (80, 16), (16, 16));
    assets.register_sprite(tiles, BEAM_HALF_LEFT, (16, 16), (16, 16));

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
