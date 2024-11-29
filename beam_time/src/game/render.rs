use beam_logic::{
    simulation::{state::BeamState, tile::BeamTile},
    tile::Tile,
};
use common::direction::Direction;
use engine::{
    assets::SpriteRef,
    drawable::sprite::Sprite,
    exports::nalgebra::Vector2,
    graphics_context::{Anchor, GraphicsContext},
};

use crate::{
    app::App,
    assets::{
        animated_sprite, BEAM_FULL_HORIZONTAL, BEAM_FULL_VERTICAL, BEAM_REFLECT_DOWN_LEFT,
        BEAM_REFLECT_DOWN_RIGHT, BEAM_REFLECT_UP_LEFT, BEAM_REFLECT_UP_RIGHT, BEAM_SPLIT_DOWN,
        BEAM_SPLIT_LEFT, BEAM_SPLIT_RIGHT, BEAM_SPLIT_UP, TILE_DELAY, TILE_DETECTOR, TILE_MIRROR_A,
        TILE_MIRROR_B, TILE_WALL,
    },
    consts::{layer, EMITTER, GALVO, HALF_BEAM, MIRROR, SPLITTER},
};

use super::SharedState;

pub trait TileAsset {
    fn asset(&self) -> Sprite;
}

pub trait BeamTileBaseSprite {
    fn base_sprite(&self, frame: u8) -> Option<Sprite>;
}

pub trait BeamStateRender {
    fn render(&mut self, ctx: &mut GraphicsContext<App>, state: &App, shared: &SharedState);
}

impl TileAsset for Tile {
    fn asset(&self) -> Sprite {
        let asset_ref = match self {
            Tile::Empty => unreachable!(),
            Tile::Detector => TILE_DETECTOR,
            Tile::Delay => TILE_DELAY,
            Tile::Emitter { rotation, active } => {
                return Sprite::new(EMITTER[*rotation as usize])
                    .uv_offset(Vector2::new(-16 * *active as i32, 0));
            }
            Tile::Mirror { rotation, .. } => MIRROR[*rotation as usize],
            Tile::Splitter { rotation, .. } => SPLITTER[*rotation as usize],
            Tile::Galvo { rotation, .. } => GALVO[*rotation as usize],
            Tile::Wall { .. } => TILE_WALL,
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
                direction,
                original_direction,
                ..
            } => animated_sprite(
                [TILE_MIRROR_A, TILE_MIRROR_B][*direction as usize],
                direction != original_direction,
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

const MIRROR_TEXTURES: [SpriteRef; 4] = [
    BEAM_REFLECT_UP_LEFT,
    BEAM_REFLECT_DOWN_RIGHT,
    BEAM_REFLECT_UP_RIGHT,
    BEAM_REFLECT_DOWN_LEFT,
];

const SPLITTER_TEXTURES: [SpriteRef; 4] = [
    BEAM_SPLIT_RIGHT,
    BEAM_SPLIT_UP,
    BEAM_SPLIT_LEFT,
    BEAM_SPLIT_DOWN,
];

impl BeamStateRender for BeamState {
    /// Renders the beam over the board.
    fn render(&mut self, ctx: &mut GraphicsContext<App>, state: &App, shared: &SharedState) {
        let half_tile = Vector2::repeat(ctx.scale_factor * shared.scale * 16.0 / 2.0);
        let size = ctx.size() + half_tile;

        let origin = shared.origin_tile(ctx);
        let frame = state.frame() as u32;

        for (pos, tile) in self.board.iter() {
            let pos = pos + origin;
            let render_pos = shared.render_pos(ctx, (pos.x as usize, pos.y as usize));

            if render_pos.x < -half_tile.x
                || render_pos.y < -half_tile.y
                || render_pos.x > size.x
                || render_pos.y > size.y
            {
                continue;
            }

            let sprite = |texture: SpriteRef| {
                Sprite::new(texture)
                    .uv_offset(Vector2::new(16 * frame as i32, 0))
                    .scale(Vector2::repeat(shared.scale))
                    .position(render_pos, Anchor::Center)
            };

            match tile {
                BeamTile::Beam {
                    direction: Direction::Left | Direction::Right,
                    ..
                } => ctx.draw(sprite(BEAM_FULL_HORIZONTAL)),
                BeamTile::Beam {
                    direction: Direction::Up | Direction::Down,
                    ..
                } => ctx.draw(sprite(BEAM_FULL_VERTICAL)),
                BeamTile::CrossBeam { .. } => {
                    ctx.draw(sprite(BEAM_FULL_HORIZONTAL));
                    ctx.draw(sprite(BEAM_FULL_VERTICAL));
                }
                BeamTile::Mirror {
                    direction, powered, ..
                } => {
                    for (idx, _) in powered.iter().enumerate().filter(|x| x.1.is_some()) {
                        let texture = MIRROR_TEXTURES[idx + direction as usize * 2];
                        ctx.draw(sprite(texture).z_index(layer::LASER * (idx == 1) as i16));
                    }
                }
                BeamTile::Splitter {
                    direction,
                    powered: Some(powered),
                } => {
                    let index = (powered as usize + direction as usize * 2) % 4;
                    ctx.draw(sprite(SPLITTER_TEXTURES[index]).z_index(layer::LASER));
                }
                BeamTile::Delay {
                    powered,
                    last_powered,
                } => {
                    for (idx, set) in [powered, last_powered].into_iter().enumerate() {
                        for dir in set.iter() {
                            ctx.draw(sprite(HALF_BEAM[dir.opposite_if(idx > 0) as usize]))
                        }
                    }
                }
                BeamTile::Galvo { powered, .. }
                | BeamTile::Wall { powered }
                | BeamTile::Detector { powered } => {
                    for dir in powered.iter() {
                        let layer = if dir == Direction::Down {
                            layer::UNDER_LASER
                        } else {
                            layer::LASER
                        };
                        ctx.draw(sprite(HALF_BEAM[dir as usize]).z_index(layer))
                    }
                }
                _ => {}
            }
        }
    }
}
