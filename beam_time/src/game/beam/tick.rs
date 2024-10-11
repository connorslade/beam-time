use engine::exports::nalgebra::Vector2;

use crate::misc::{
    direction::{Direction, Directions},
    map::Map,
};

use super::{opposite_if, BeamState, BeamTile, MIRROR_REFLECTIONS};

impl BeamState {
    pub fn tick(&mut self) {
        // To avoid issues that would arise from modifying the board in place, a
        // copy is made to store the new state.
        let mut working_board = self.board.clone();

        // todo: maybe a better way to do this?
        for (pos, tile) in self.board.iter() {
            if let BeamTile::Delay { .. } = tile {
                let (powered, last_powered) = working_board.get_mut(pos).delay_mut();
                *last_powered = *powered;
            }
        }

        for (pos, tile) in self.board.iter() {
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
                BeamTile::Beam {
                    direction,
                    distance,
                } => {
                    if distance == 255 {
                        working_board.remove(pos);
                        continue;
                    }

                    if self.source_gone(pos, direction) {
                        working_board.remove(pos);
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
                            let tile = working_board.get_mut(pos);
                            if let BeamTile::CrossBeam { .. } = tile {
                                let direction = directions[1 - idx];
                                *tile = BeamTile::Beam {
                                    direction,
                                    distance: 0,
                                };
                            } else {
                                working_board.remove(pos);
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
                            working_board.get_mut(pos).mirror_mut().2[idx] = None;
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
                    let direction = opposite_if(MIRROR_REFLECTIONS[powered as usize], !direction);
                    self.power(&mut working_board, pos, powered);
                    self.power(&mut working_board, pos, direction);

                    if self.source_gone(pos, powered) {
                        *working_board.get_mut(pos).splitter_mut() = None;
                    }
                }
                // Galvos change the rotation of the mirror they are
                // pointing into when powered by a beam.
                BeamTile::Galvo { direction, powered } => {
                    self.track_powered(working_board.get_mut(pos).directions_mut(), pos);

                    let BeamTile::Mirror {
                        direction: mirror_direction,
                        original_direction,
                        powered: powered_sides,
                    } = working_board.get_mut(direction.offset(pos))
                    else {
                        continue;
                    };

                    let desired_direction =
                        *original_direction ^ powered.any_but(direction.opposite());
                    (*mirror_direction != desired_direction).then(|| *powered_sides = [None; 2]);
                    *mirror_direction = desired_direction;
                }
                BeamTile::Delay { last_powered, .. } => {
                    for dir in last_powered.iter() {
                        self.power(&mut working_board, pos, dir);
                    }

                    let (powered, _) = working_board.get_mut(pos).delay_mut();
                    self.track_powered(powered, pos);
                }
                BeamTile::Wall { .. } | BeamTile::Detector { .. } => {
                    self.track_powered(working_board.get_mut(pos).directions_mut(), pos)
                }
                _ => {}
            }
        }

        self.board = working_board;
    }

    /// Checks if a given tile is providing power in the given direction.
    fn source_gone(&self, pos: Vector2<i32>, direction: Direction) -> bool {
        let source = direction.opposite().offset(pos);
        let source_tile = self.board.get(source);
        !source_tile.is_powered() || !source_tile.power_direction().contains(direction)
    }

    fn track_powered(&self, directions: &mut Directions, pos: Vector2<i32>) {
        directions
            .iter()
            .filter(|&x| self.source_gone(pos, x))
            .for_each(|x| *directions &= !Directions::from(x));
    }

    /// Powers a tile in the given direction.
    fn power(&self, working_board: &mut Map<BeamTile>, pos: Vector2<i32>, direction: Direction) {
        let pos = direction.offset(pos);
        let tile = working_board.get_mut(pos);

        match tile {
            BeamTile::Empty => {
                *tile = BeamTile::Beam {
                    direction,
                    distance: 0,
                }
            }
            BeamTile::Beam { direction: dir, .. } if dir.is_perpendicular(direction) => {
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
                powered,
                direction: dir,
            } if direction != dir.opposite() => *powered |= direction,
            BeamTile::Wall { powered }
            | BeamTile::Detector { powered }
            | BeamTile::Delay { powered, .. } => *powered |= direction,
            _ => {}
        }
    }
}
