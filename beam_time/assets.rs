use engine::{
    assets::{AudioRef, FontRef, SpriteRef, constructor::AssetConstructor, font::FontDescriptor},
    define_refs,
    drawable::sprite::Sprite,
    exports::{nalgebra::Vector2, winit::window::Icon},
};
use image::{RgbaImage, imageops::FilterType};

use crate::util::{include_asset, include_atlas};

define_refs! {
    FontRef => {
        UNDEAD_FONT,
        ALAGARD_FONT
    },
    AudioRef => {
        BUTTON_HOVER,
        BUTTON_CLICK
    },
    SpriteRef => {
        CONFETTI_PARTICLES,
        BIG_RIGHT_ARROW,
        LEFT_ARROW,
        RIGHT_ARROW,
        DOWN_ARROW,
        TRASH,
        DUPLICATE,
        EDIT,
        RESET,
        SLIDER_HANDLE,
        CHECK,
        COLLAPSE_UP,
        COLLAPSE_DOWN,
        KEYBOARD_BUTTON,
        TOGGLE_ACTIVE,
        TOGGLE_INACTIVE,
        X,

        EMPTY_TILE_A,
        EMPTY_TILE_B,
        PERMANENT_TILE_A,
        PERMANENT_TILE_B,
        DYNAMIC_TILE_A,
        DYNAMIC_TILE_B,
        DYNAMIC_TILE_OUTLINE,

        TILE_WALL,
        TILE_DETECTOR,
        TILE_DELAY,
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
    Sprite::new(texture).uv_offset(Vector2::new(offset as i32, 0))
}

pub fn init(assets: &mut AssetConstructor) {
    assets.register_audio(BUTTON_CLICK, include_asset!("sounds/button-hover.mp3"));
    assets.register_audio(BUTTON_HOVER, include_asset!("sounds/button-click.mp3"));

    let interface = assets.register_atlas(include_atlas!("textures/interface.png"));
    assets.register_sprite(interface, CONFETTI_PARTICLES, (32, 0), (3, 3));
    assets.register_sprite(interface, BIG_RIGHT_ARROW, (0, 0), (10, 9));
    assets.register_sprite(interface, LEFT_ARROW, (0, 10), (3, 6));
    assets.register_sprite(interface, RIGHT_ARROW, (4, 10), (3, 6));
    assets.register_sprite(interface, DOWN_ARROW, (9, 12), (5, 4));
    assets.register_sprite(interface, TRASH, (24, 8), (7, 9));
    assets.register_sprite(interface, DUPLICATE, (16, 8), (7, 9));
    assets.register_sprite(interface, EDIT, (32, 8), (8, 9));
    assets.register_sprite(interface, RESET, (24, 18), (8, 9));
    assets.register_sprite(interface, SLIDER_HANDLE, (19, 18), (3, 6));
    assets.register_sprite(interface, CHECK, (19, 26), (5, 5));
    assets.register_sprite(interface, COLLAPSE_UP, (16, 0), (5, 7));
    assets.register_sprite(interface, COLLAPSE_DOWN, (22, 0), (5, 7));
    assets.register_sprite(interface, KEYBOARD_BUTTON, (0, 16), (9, 13));
    assets.register_sprite(interface, TOGGLE_ACTIVE, (10, 26), (8, 5));
    assets.register_sprite(interface, TOGGLE_INACTIVE, (10, 19), (8, 5));
    assets.register_sprite(interface, X, (33, 20), (5, 6));

    let tiles = assets.register_atlas(include_atlas!("textures/tilemap.png"));
    assets.register_sprite(tiles, EMPTY_TILE_A, (0, 288), (16, 16));
    assets.register_sprite(tiles, EMPTY_TILE_B, (16, 288), (16, 16));
    assets.register_sprite(tiles, PERMANENT_TILE_A, (32, 288), (16, 16));
    assets.register_sprite(tiles, PERMANENT_TILE_B, (48, 288), (16, 16));
    assets.register_sprite(tiles, DYNAMIC_TILE_A, (96, 304), (16, 16));
    assets.register_sprite(tiles, DYNAMIC_TILE_B, (112, 304), (16, 16));
    assets.register_sprite(tiles, DYNAMIC_TILE_OUTLINE, (96, 288), (16, 16));

    assets.register_sprite(tiles, TILE_WALL, (0, 208), (16, 16));
    assets.register_sprite(tiles, TILE_DETECTOR, (0, 192), (16, 16));
    assets.register_sprite(tiles, TILE_DELAY, (0, 256), (16, 16));
    assets.register_sprite(tiles, TILE_MIRROR_A, (0, 64), (16, 16));
    assets.register_sprite(tiles, TILE_MIRROR_B, (0, 80), (16, 16));
    assets.register_sprite(tiles, TILE_SPLITTER_A, (0, 160), (16, 16));
    assets.register_sprite(tiles, TILE_SPLITTER_B, (0, 176), (16, 16));
    assets.register_sprite(tiles, TILE_GALVO_UP, (64, 224), (16, 16));
    assets.register_sprite(tiles, TILE_GALVO_RIGHT, (0, 224), (16, 16));
    assets.register_sprite(tiles, TILE_GALVO_DOWN, (64, 240), (16, 16));
    assets.register_sprite(tiles, TILE_GALVO_LEFT, (0, 240), (16, 16));
    assets.register_sprite(tiles, TILE_EMITTER_UP, (16, 128), (16, 16));
    assets.register_sprite(tiles, TILE_EMITTER_DOWN, (16, 144), (16, 16));
    assets.register_sprite(tiles, TILE_EMITTER_LEFT, (16, 112), (16, 16));
    assets.register_sprite(tiles, TILE_EMITTER_RIGHT, (16, 96), (16, 16));

    assets.register_sprite(tiles, BEAM_FULL_HORIZONTAL, (16, 32), (16, 16));
    assets.register_sprite(tiles, BEAM_FULL_VERTICAL, (16, 48), (16, 16));
    assets.register_sprite(tiles, BEAM_REFLECT_UP_LEFT, (80, 32), (16, 16));
    assets.register_sprite(tiles, BEAM_REFLECT_DOWN_LEFT, (80, 48), (16, 16));
    assets.register_sprite(tiles, BEAM_REFLECT_UP_RIGHT, (80, 80), (16, 16));
    assets.register_sprite(tiles, BEAM_REFLECT_DOWN_RIGHT, (80, 64), (16, 16));
    assets.register_sprite(tiles, BEAM_SPLIT_UP, (80, 160), (16, 16));
    assets.register_sprite(tiles, BEAM_SPLIT_RIGHT, (80, 176), (16, 16));
    assets.register_sprite(tiles, BEAM_SPLIT_DOWN, (80, 192), (16, 16));
    assets.register_sprite(tiles, BEAM_SPLIT_LEFT, (80, 208), (16, 16));
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

pub fn icon(size: u32) -> Icon {
    let image = image::load_from_memory(include_asset!("textures/icon.png"))
        .unwrap()
        .resize(size, size, FilterType::Nearest);
    Icon::from_rgba(image.to_rgba8().to_vec(), size, size).unwrap()
}
