use beam_logic::{simulation::tile::BeamTile, tile::Tile};
use engine::{assets::SpriteRef, drawable::sprite::Sprite, exports::nalgebra::Vector2};

use crate::assets::{
    BEAM_HALF_DOWN, BEAM_HALF_LEFT, BEAM_HALF_RIGHT, BEAM_HALF_UP, TILE_DELAY, TILE_DETECTOR,
    TILE_EMITTER_DOWN, TILE_EMITTER_LEFT, TILE_EMITTER_RIGHT, TILE_EMITTER_UP, TILE_GALVO_DOWN,
    TILE_GALVO_LEFT, TILE_GALVO_RIGHT, TILE_GALVO_UP, TILE_MIRROR_A, TILE_MIRROR_B,
    TILE_SPLITTER_A, TILE_SPLITTER_B, TILE_WALL, animated_sprite,
};

pub const GALVO: [SpriteRef; 4] = [
    TILE_GALVO_UP,
    TILE_GALVO_RIGHT,
    TILE_GALVO_DOWN,
    TILE_GALVO_LEFT,
];

pub const EMITTER: [SpriteRef; 4] = [
    TILE_EMITTER_UP,
    TILE_EMITTER_RIGHT,
    TILE_EMITTER_DOWN,
    TILE_EMITTER_LEFT,
];

pub const SPLITTER: [SpriteRef; 2] = [TILE_SPLITTER_A, TILE_SPLITTER_B];
pub const MIRROR: [SpriteRef; 2] = [TILE_MIRROR_A, TILE_MIRROR_B];

pub const HALF_BEAM: [SpriteRef; 4] = [
    BEAM_HALF_UP,
    BEAM_HALF_RIGHT,
    BEAM_HALF_DOWN,
    BEAM_HALF_LEFT,
];

pub trait TileAsset {
    fn asset(&self) -> Sprite;
}

pub trait BeamTileBaseSprite {
    fn base_sprite(&self, frame: u8) -> Option<Sprite>;
}

impl TileAsset for Tile {
    fn asset(&self) -> Sprite {
        let asset_ref = match self {
            Tile::Empty => unreachable!(),
            Tile::Detector { .. } => TILE_DETECTOR,
            Tile::Delay => TILE_DELAY,
            Tile::Emitter {
                rotation, active, ..
            } => {
                return Sprite::new(EMITTER[*rotation as usize])
                    .uv_offset(Vector2::new(-16 * *active as i32, 0));
            }
            Tile::Mirror { rotation, .. } => MIRROR[*rotation as usize],
            Tile::Splitter { rotation, .. } => SPLITTER[*rotation as usize],
            Tile::Galvo { rotation, .. } => GALVO[*rotation as usize],
            Tile::Wall => TILE_WALL,
        };

        Sprite::new(asset_ref)
    }
}

impl BeamTileBaseSprite for BeamTile {
    /// Overwrites the texture of a tile for rendering purposes.
    fn base_sprite(&self, frame: u8) -> Option<Sprite> {
        Some(match self {
            BeamTile::Emitter { direction, active } => {
                animated_sprite(EMITTER[*direction as usize], *active, frame)
            }
            BeamTile::Detector { powered } => animated_sprite(TILE_DETECTOR, powered.any(), frame),
            BeamTile::Delay { powered, .. } => animated_sprite(TILE_DELAY, powered.any(), frame),
            BeamTile::Mirror {
                galvoed, direction, ..
            } => animated_sprite(
                [TILE_MIRROR_A, TILE_MIRROR_B][(direction ^ galvoed.any()) as usize],
                galvoed.any(),
                frame,
            ),
            BeamTile::Galvo { direction, powered } if powered.any_but(direction.opposite()) => {
                animated_sprite(GALVO[*direction as usize], true, frame)
            }
            BeamTile::Splitter { direction, powered } if powered.any() => {
                animated_sprite(SPLITTER[*direction as usize], true, frame)
            }
            _ => return None,
        })
    }
}
