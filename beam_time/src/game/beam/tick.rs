use engine::exports::nalgebra::Vector2;

use crate::misc::direction::Direction;

use super::{opposite_if, BeamState, BeamTile, MIRROR_REFLECTIONS};

impl BeamState {
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
                    BeamTile::Emitter {
                        direction,
                        active: true,
                    } => {
                        self.power(&mut working_board, pos, direction);
                    }
                    // A beam will send out power in the direction it is facing
                    // and will destroy itself if it is no longer receiving
                    // power from the opposite direction.
                    BeamTile::Beam { direction } => {
                        // Unwrap is used here, because a beam can't be created
                        // from an edge tile, only from emitters.
                        if self.source_gone(pos, direction) {
                            working_board[index] = BeamTile::Empty;
                        }

                        self.power(&mut working_board, pos, direction);
                    }
                    // When two perpendicular beams meet, they form a crossbeam,
                    // which continues to propagate the beams in both
                    // directions. If either beam is lost, the crossbeam will
                    // turn into a beam in the remaining direction.
                    BeamTile::CrossBeam { directions } => {
                        for (idx, &direction) in directions.iter().enumerate() {
                            self.power(&mut working_board, pos, direction);

                            if self.source_gone(pos, direction) {
                                let tile = &mut working_board[index];
                                if let BeamTile::CrossBeam { .. } = tile {
                                    let direction = directions[1 - idx];
                                    *tile = BeamTile::Beam { direction }
                                } else {
                                    *tile = BeamTile::Empty;
                                }
                            }
                        }
                    }
                    // Mirrors will reflect beams based on the
                    // MIRROR_REFLECTIONS table. Powered is an array of two
                    // because rotation state of the mirror has a top and
                    // bottom, which can each be powered independently.
                    BeamTile::Mirror {
                        direction, powered, ..
                    } => {
                        for (idx, powered) in powered.iter().enumerate() {
                            let Some(powered) = *powered else { continue };

                            let direction =
                                opposite_if(MIRROR_REFLECTIONS[powered as usize], !direction);
                            self.power(&mut working_board, pos, direction);

                            if self.source_gone(pos, powered) {
                                // We can safely unwrap here, because the
                                // current tile is known to be a mirror.
                                working_board[index].mirror_mut().unwrap().2[idx] = None;
                            }
                        }
                    }
                    // Splitters effectively act as mirrors that also pass the
                    // existing beam through. One difference is that they only
                    // take in one input beam at a time.
                    BeamTile::Splitter {
                        direction,
                        powered: Some(powered),
                    } => {
                        let direction =
                            opposite_if(MIRROR_REFLECTIONS[powered as usize], !direction);
                        self.power(&mut working_board, pos, powered);
                        self.power(&mut working_board, pos, direction);

                        if self.source_gone(pos, powered) {
                            *working_board[index].splitter_mut().unwrap() = None;
                        }
                    }
                    // Galvos change the rotation of the mirror they are
                    // pointing into when powered by a beam.
                    BeamTile::Galvo { direction, powered } => {
                        if let Some(powered) = powered {
                            if self.source_gone(pos, powered) {
                                *working_board[index].galvo_mut().unwrap() = None;
                            }
                        }

                        let pointing = direction
                            .offset(self.size, pos)
                            .and_then(|x| working_board[self.to_index(x)].mirror_mut());
                        let Some((original_direction, direction, powered_sides)) = pointing else {
                            continue;
                        };

                        let desired_direction = original_direction ^ powered.is_some();
                        (*direction != desired_direction).then(|| *powered_sides = [None; 2]);
                        *direction = desired_direction;
                    }
                    _ => {}
                }
            }
        }

        self.board = working_board;
    }

    /// Converts a position to an index in the board.
    fn to_index(&self, pos: Vector2<usize>) -> usize {
        pos.y * self.size.x + pos.x
    }

    /// Checks if a given tile is providing power in the given direction.
    fn source_gone(&self, pos: Vector2<usize>, direction: Direction) -> bool {
        let source = direction.opposite().offset(self.size, pos).unwrap();
        let source_tile = self.board[self.to_index(source)];
        !source_tile.is_powered() || !source_tile.power_direction().contains(direction)
    }

    /// Powers a tile in the given direction.
    fn power(&self, working_board: &mut [BeamTile], pos: Vector2<usize>, direction: Direction) {
        let Some(index) = direction.offset(self.size, pos).map(|x| self.to_index(x)) else {
            return;
        };

        let tile = &mut working_board[index];
        match tile {
            BeamTile::Empty => *tile = BeamTile::Beam { direction },
            BeamTile::Beam { direction: dir } if dir.is_perpendicular(direction) => {
                *tile = BeamTile::CrossBeam {
                    directions: [*dir, direction],
                }
            }
            BeamTile::Mirror {
                direction: dir,
                powered,
                ..
            } => {
                if *dir {
                    powered[matches!(direction, Direction::Up | Direction::Right) as usize] =
                        Some(direction);
                } else {
                    powered[matches!(direction, Direction::Up | Direction::Left) as usize] =
                        Some(direction);
                }
            }
            BeamTile::Splitter { powered, .. } if powered.is_none() => *powered = Some(direction),
            BeamTile::Galvo {
                direction: dir,
                powered,
                ..
            } if dir.opposite() != direction => *powered = Some(direction),
            _ => {}
        }
    }
}
