use engine::{
    assets::SpriteRef,
    drawable::sprite::Sprite,
    exports::nalgebra::Vector2,
    graphics_context::{Anchor, GraphicsContext},
};
use tile::BeamTile;

use crate::{
    app::App,
    assets::{
        BEAM_FULL_HORIZONTAL, BEAM_FULL_VERTICAL, BEAM_REFLECT_DOWN_LEFT, BEAM_REFLECT_DOWN_RIGHT,
        BEAM_REFLECT_UP_LEFT, BEAM_REFLECT_UP_RIGHT, BEAM_SPLIT_DOWN, BEAM_SPLIT_LEFT,
        BEAM_SPLIT_RIGHT, BEAM_SPLIT_UP,
    },
    consts::{layer, HALF_BEAM},
    misc::direction::{Direction, Directions},
};

use super::{board::Board, tile::Tile, SharedState};

mod tick;
mod tile;

const MIRROR_REFLECTIONS: [Direction; 4] = [
    Direction::Left,
    Direction::Down,
    Direction::Right,
    Direction::Up,
];

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

pub struct BeamState {
    pub board: Vec<BeamTile>,
    pub size: Vector2<usize>,
}

impl BeamState {
    /// Creates a new BeamState from a Board by converting Tiles into their
    /// BeamTile counterparts.
    pub fn new(board: &Board) -> Self {
        let size = board.size;
        let board = board
            .tiles
            .iter()
            .map(|x| match x {
                Tile::Empty => BeamTile::Empty,
                Tile::Emitter { rotation, active } => BeamTile::Emitter {
                    direction: *rotation,
                    active: *active,
                },
                Tile::Detector => BeamTile::Detector {
                    powered: Directions::empty(),
                },
                Tile::Delay => BeamTile::Delay {
                    powered: Directions::empty(),
                    last_powered: Directions::empty(),
                },
                Tile::Mirror { rotation } => BeamTile::Mirror {
                    direction: *rotation,
                    original_direction: *rotation,
                    powered: [None; 2],
                },
                Tile::Splitter { rotation } => BeamTile::Splitter {
                    direction: *rotation,
                    powered: None,
                },
                Tile::Galvo { rotation } => BeamTile::Galvo {
                    direction: *rotation,
                    powered: Directions::empty(),
                },
                Tile::Wall { .. } => BeamTile::Wall {
                    powered: Directions::empty(),
                },
            })
            .collect();

        Self { board, size }
    }

    /// Renders the beam over the board.
    pub fn render(&mut self, ctx: &mut GraphicsContext<App>, state: &App, shared: &SharedState) {
        let frame = state.frame() as u32;

        for x in 0..self.size.x {
            for y in 0..self.size.y {
                let index = y * self.size.x + x;
                let beam = self.board[index];
                let pos = shared.tile_pos(ctx, self.size, Vector2::new(x, y));

                let sprite = |texture: SpriteRef| {
                    Sprite::new(texture)
                        .uv_offset(Vector2::new(16 * frame, 0))
                        .scale(Vector2::repeat(shared.scale), Anchor::Center)
                        .position(pos, Anchor::Center)
                };

                match beam {
                    BeamTile::Beam {
                        direction: Direction::Left | Direction::Right,
                    } => ctx.draw(sprite(BEAM_FULL_HORIZONTAL)),
                    BeamTile::Beam {
                        direction: Direction::Up | Direction::Down,
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
                                ctx.draw(sprite(HALF_BEAM[opposite_if(dir, idx > 0) as usize]))
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
}

fn opposite_if(direction: Direction, condition: bool) -> Direction {
    if condition {
        direction.opposite()
    } else {
        direction
    }
}
