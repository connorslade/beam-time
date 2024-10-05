use std::f32::consts::{PI, TAU};

use engine::{
    drawable::sprite::Sprite,
    exports::nalgebra::Vector2,
    graphics_context::{Anchor, GraphicsContext},
};

use crate::{
    assets::{BEAM, MIRROR_BEAM},
    misc::direction::Direction,
};

use super::{board::Board, tile::Tile};

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

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum BeamTile {
    #[default]
    Empty,
    Wall,
    Beam {
        direction: Direction,
    },
    Emitter {
        direction: Direction,
    },
    Mirror {
        direction: bool,
        powered: [Option<Direction>; 2],
    },
    Splitter {
        direction: bool,
        powered: bool,
    },
}

impl BeamState {
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

    pub fn tick(&mut self) {
        let to_index = |pos: Vector2<usize>| pos.y * self.size.x + pos.x;
        let power = |working_board: &mut [BeamTile], index: usize, direction: Direction| {
            let tile = &mut working_board[index];
            match tile {
                BeamTile::Empty => *tile = BeamTile::Beam { direction },
                BeamTile::Beam { direction: dir } if dir.is_perpendicular(direction) => todo!(),
                BeamTile::Mirror {
                    direction: dir,
                    powered,
                } => {
                    if *dir {
                        powered[matches!(direction, Direction::Up | Direction::Right) as usize] =
                            Some(direction);
                    } else {
                        powered[matches!(direction, Direction::Up | Direction::Left) as usize] =
                            Some(direction);
                    }
                }
                BeamTile::Splitter { powered, .. } => *powered = true,
                _ => {}
            }
        };

        let mut working_board = self.board.clone();
        for y in 0..self.size.y {
            for x in 0..self.size.x {
                let pos = Vector2::new(x, y);
                let index = to_index(pos);
                let tile = self.board[index];

                match tile {
                    BeamTile::Empty => {}
                    BeamTile::Emitter { direction } => {
                        if let Some(sink) = direction.offset(self.size, pos) {
                            power(&mut working_board, to_index(sink), direction);
                        }
                    }
                    BeamTile::Beam { direction } => {
                        if let Some(source) = direction.opposite().offset(self.size, pos) {
                            let source_tile = self.board[to_index(source)];
                            if !source_tile.is_powered()
                                || !source_tile.power_direction().contains(&direction)
                            {
                                working_board[index] = BeamTile::Empty;
                            }
                        } else {
                            working_board[index] = BeamTile::Empty;
                        }

                        if let Some(sink) = direction.offset(self.size, pos) {
                            power(&mut working_board, to_index(sink), direction);
                        }
                    }
                    BeamTile::Mirror { direction, powered } => {
                        let mut mirror_reflection = |dir: bool, powered: Option<Direction>| {
                            if let Some(powered) = powered {
                                let mut direction = MIRROR_REFLECTIONS[powered as usize];
                                if !dir {
                                    direction = direction.opposite();
                                }

                                if let Some(sink) = direction.offset(self.size, pos) {
                                    power(&mut working_board, to_index(sink), direction);
                                }
                            }
                        };

                        mirror_reflection(direction, powered[0]);
                        mirror_reflection(direction, powered[1]);
                    }
                    _ => {}
                }
            }
        }

        self.board = working_board;
    }

    pub fn render<App>(&mut self, ctx: &mut GraphicsContext<App>) {
        let tile_size = 16.0 * 4.0 * ctx.scale_factor;
        let size = self.size.map(|x| x as f32) * tile_size;

        for x in 0..self.size.x {
            for y in 0..self.size.y {
                let index = y * self.size.x + x;
                let beam = self.board[index];

                let pos = ctx.center() - Vector2::new(x as f32 * tile_size, y as f32 * tile_size)
                    + size / 2.0
                    - Vector2::repeat(tile_size / 2.0);

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

impl BeamTile {
    pub fn is_powered(&self) -> bool {
        match self {
            Self::Emitter { .. } | Self::Beam { .. } => true,
            Self::Mirror { powered, .. } => powered[0].is_some() || powered[1].is_some(),
            _ => false,
        }
    }

    // todo: dont use vec? bitset?
    pub fn power_direction(&self) -> Vec<Direction> {
        match self {
            Self::Beam { direction } | Self::Emitter { direction } => vec![*direction],
            Self::Mirror { direction, powered } => {
                let mut out = Vec::new();

                let mut mirror_reflection = |dir: bool, powered: Option<Direction>| {
                    if let Some(powered) = powered {
                        let mut direction = MIRROR_REFLECTIONS[powered as usize];
                        if !dir {
                            direction = direction.opposite();
                        }

                        out.push(direction);
                    }
                };

                mirror_reflection(*direction, powered[0]);
                mirror_reflection(*direction, powered[1]);

                out
            }
            _ => Vec::new(),
        }
    }
}
