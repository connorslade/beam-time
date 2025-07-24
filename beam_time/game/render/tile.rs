use beam_logic::{simulation::tile::BeamTile, tile::Tile};
use engine::{drawable::sprite::Sprite, exports::nalgebra::Vector2};

use crate::{
    assets::{TILE_DELAY, TILE_DETECTOR, TILE_MIRROR_A, TILE_MIRROR_B, TILE_WALL, animated_sprite},
    consts::{EMITTER, GALVO, MIRROR, SPLITTER},
};

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
                [TILE_MIRROR_A, TILE_MIRROR_B][(direction ^ galvoed.odd_count()) as usize],
                galvoed.any(),
                frame,
            ),
            BeamTile::Galvo { direction, powered } if powered.any_but(direction.opposite()) => {
                animated_sprite(GALVO[*direction as usize], true, frame)
            }
            BeamTile::Splitter {
                direction,
                powered: Some(..),
            } => animated_sprite(SPLITTER[*direction as usize], true, frame),
            _ => return None,
        })
    }
}
