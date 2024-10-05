use engine::exports::nalgebra::Vector2;

use crate::misc::direction::Direction;

use super::{opposite_if, BeamState, BeamTile, MIRROR_REFLECTIONS};

impl BeamState {
    fn to_index(&self, pos: Vector2<usize>) -> usize {
        pos.y * self.size.x + pos.x
    }

    pub fn tick(&mut self) {
        let mut working_board = self.board.clone();
        for y in 0..self.size.y {
            for x in 0..self.size.x {
                let pos = Vector2::new(x, y);
                let index = self.to_index(pos);
                let tile = self.board[index];

                match tile {
                    BeamTile::Empty => {}
                    BeamTile::Emitter { direction } => {
                        if let Some(sink) = direction.offset(self.size, pos) {
                            power(&mut working_board, self.to_index(sink), direction);
                        }
                    }
                    BeamTile::Beam { direction } => {
                        if let Some(source) = direction.opposite().offset(self.size, pos) {
                            let source_tile = self.board[self.to_index(source)];
                            if !source_tile.is_powered()
                                || !source_tile.power_direction().contains(direction)
                            {
                                working_board[index] = BeamTile::Empty;
                            }
                        } else {
                            working_board[index] = BeamTile::Empty;
                        }

                        if let Some(sink) = direction.offset(self.size, pos) {
                            power(&mut working_board, self.to_index(sink), direction);
                        }
                    }
                    BeamTile::Mirror { direction, powered } => {
                        let mut mirror_reflection = |dir: bool, powered: Option<Direction>| {
                            if let Some(powered) = powered {
                                let direction =
                                    opposite_if(MIRROR_REFLECTIONS[powered as usize], !dir);
                                if let Some(sink) = direction.offset(self.size, pos) {
                                    power(&mut working_board, self.to_index(sink), direction);
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
}

fn power(working_board: &mut [BeamTile], index: usize, direction: Direction) {
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
}
