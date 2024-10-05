use std::f32::consts::PI;

use engine::{
    drawable::sprite::Sprite,
    exports::nalgebra::Vector2,
    graphics_context::{Anchor, GraphicsContext},
};
use tile::BeamTile;

use crate::{
    assets::{BEAM, MIRROR_BEAM},
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
    board: Vec<BeamTile>,
    size: Vector2<usize>,
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
                    powered: [None; 2],
                },
                Tile::Splitter { rotation } => BeamTile::Splitter {
                    direction: *rotation,
                    powered: false,
                },
                Tile::Galvo => todo!(),
                Tile::Wall => BeamTile::Wall,
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

                match beam {
                    BeamTile::Beam { direction } => {
                        let sprite = Sprite::new(BEAM)
                            .scale(Vector2::repeat(4.0), Anchor::Center)
                            .position(pos, Anchor::Center)
                            .rotate(direction.to_angle(), Anchor::Center);
                        ctx.draw(sprite);
                    }
                    BeamTile::Mirror { direction, powered } => {
                        for i in powered
                            .iter()
                            .enumerate()
                            .filter_map(|(i, x)| x.is_some().then_some(i))
                        {
                            let rotation = PI * i as f32 - (PI / 2.0) * direction as u8 as f32;
                            let sprite = Sprite::new(MIRROR_BEAM)
                                .scale(Vector2::repeat(4.0), Anchor::Center)
                                .position(pos, Anchor::Center)
                                .rotate(rotation, Anchor::Center);
                            ctx.draw(sprite);
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
