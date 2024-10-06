use std::f32::consts::PI;

use engine::{
    assets::SpriteRef,
    drawable::sprite::Sprite,
    exports::nalgebra::Vector2,
    graphics_context::{Anchor, GraphicsContext},
};
use tile::BeamTile;

use crate::{
    assets::{BEAM, CROSS_BEAM, MIRROR_BEAM, SPLITTER_BEAM},
    misc::direction::Direction,
};

use super::{board::Board, tile::Tile, tile_pos};

mod tick;
mod tile;

const MIRROR_REFLECTIONS: [Direction; 4] = [
    Direction::Left,
    Direction::Down,
    Direction::Right,
    Direction::Up,
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
                Tile::Emitter { rotation } => BeamTile::Emitter {
                    direction: *rotation,
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
                    powered: None,
                },
                Tile::Wall { .. } => BeamTile::Wall,
            })
            .collect();

        Self { board, size }
    }

    /// Renders the beam over the board.
    pub fn render<App>(&mut self, ctx: &mut GraphicsContext<App>) {
        for x in 0..self.size.x {
            for y in 0..self.size.y {
                let index = y * self.size.x + x;
                let beam = self.board[index];
                let pos = tile_pos(ctx, self.size, Vector2::new(x, y));

                let sprite = |texture: SpriteRef| {
                    Sprite::new(texture)
                        .scale(Vector2::repeat(4.0), Anchor::Center)
                        .position(pos, Anchor::Center)
                };

                match beam {
                    BeamTile::Beam { direction } => {
                        ctx.draw(sprite(BEAM).rotate(direction.to_angle(), Anchor::Center))
                    }
                    BeamTile::CrossBeam { .. } => ctx.draw(sprite(CROSS_BEAM)),
                    BeamTile::Mirror {
                        direction, powered, ..
                    } => {
                        for i in powered
                            .iter()
                            .enumerate()
                            .filter_map(|(i, x)| x.is_some().then_some(i))
                        {
                            let rotation = PI * i as f32 - (PI / 2.0) * direction as u8 as f32;
                            ctx.draw(sprite(MIRROR_BEAM).rotate(rotation, Anchor::Center));
                        }
                    }
                    BeamTile::Splitter {
                        direction,
                        powered: Some(powered),
                    } => {
                        let rotation = match powered {
                            Direction::Right => PI / 2.0,
                            Direction::Left => -PI / 2.0,
                            x => x.to_angle(),
                        } + direction as u8 as f32 * PI;
                        ctx.draw(sprite(SPLITTER_BEAM).rotate(rotation, Anchor::Center));
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
