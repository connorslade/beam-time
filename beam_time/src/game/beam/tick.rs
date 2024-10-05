use engine::exports::nalgebra::Vector2;

use crate::misc::direction::Direction;

use super::{opposite_if, BeamState, BeamTile, MIRROR_REFLECTIONS};

impl BeamState {
    fn to_index(&self, pos: Vector2<usize>) -> usize {
        pos.y * self.size.x + pos.x
    }

    pub fn tick(&mut self) {
        // To avoid issues that would arise from modifying the board in place, a
        // copy is made to store the new state.
        let mut working_board = self.board.clone();

        for y in 0..self.size.y {
            for x in 0..self.size.x {
                let pos = Vector2::new(x, y);
                let index = self.to_index(pos);
                let tile = self.board[index];

                match tile {
                    BeamTile::Empty => {}
                    // Emitters send out a constant beam in the direction they
                    // are facing.
                    BeamTile::Emitter { direction } => {
                        if let Some(sink) = direction.offset(self.size, pos) {
                            power(&mut working_board, self.to_index(sink), direction);
                        }
                    }
                    // A beam will send out power in the direction it is facing
                    // and will destroy itself if it is no longer receiving
                    // power from the opposite direction.
                    BeamTile::Beam { direction } => {
                        // Unwrap is used here, because a beam can't be created
                        // from an edge tile, only from emitters.
                        let source = direction.opposite().offset(self.size, pos).unwrap();
                        let source_tile = self.board[self.to_index(source)];
                        if !source_tile.is_powered()
                            || !source_tile.power_direction().contains(direction)
                        {
                            working_board[index] = BeamTile::Empty;
                        }

                        if let Some(sink) = direction.offset(self.size, pos) {
                            power(&mut working_board, self.to_index(sink), direction);
                        }
                    }
                    // Mirrors will reflect beams based on the
                    // MIRROR_REFLECTIONS table.
                    BeamTile::Mirror { direction, powered } => {
                        for &powered in powered.iter().flatten() {
                            let direction =
                                opposite_if(MIRROR_REFLECTIONS[powered as usize], !direction);
                            if let Some(sink) = direction.offset(self.size, pos) {
                                power(&mut working_board, self.to_index(sink), direction);
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        self.board = working_board;
    }
}

/// Powers a tile in the given direction.
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
