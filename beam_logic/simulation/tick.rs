use nalgebra::Vector2;

use common::{
    direction::{Direction, Directions},
    map::Map,
};

use super::{MIRROR_REFLECTIONS, state::BeamState, tile::BeamTile};

// TODO: Update board in stages to avoid issues due to undefined update order?
// - Delay
// - Mirror / Splitter
// - Beam / Cross Beam
// - Galvo / Emitter Detector

impl BeamState {
    pub fn tick(&mut self) {
        if self.level.is_some() {
            let hash = self.hash();
            let level = self.level.as_mut().unwrap();
            level.tick(hash, &mut self.board);

            // If the level has been completed or failed, don't continue
            // updating the board
            if level.result.is_some() {
                return;
            }
        }

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
                        if let BeamTile::CrossBeam { directions } = working_board.get(pos) {
                            let index = directions.iter().position(|&x| x == direction).unwrap();
                            let direction = directions[1 - index];

                            let new_tile = BeamTile::Beam {
                                direction,
                                distance,
                            };
                            working_board.set(pos, new_tile);
                        } else {
                            working_board.remove(pos);
                        }
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
                    galvoed,
                    powered,
                    direction,
                } => {
                    for (idx, powered) in powered.iter().enumerate() {
                        let Some(powered) = *powered else { continue };

                        let direction = MIRROR_REFLECTIONS[powered as usize]
                            .opposite_if(!(direction ^ galvoed.any()));
                        self.power(&mut working_board, pos, direction);

                        if self.source_gone(pos, powered) {
                            working_board.get_mut(pos).mirror_mut().2[idx] = None;
                        }
                    }
                }
                // Splitters effectively act as mirrors that also pass the
                // existing beam through. One difference is that they only
                // take in one input beam at a time.
                BeamTile::Splitter { direction, powered } => {
                    for powered in powered.iter() {
                        let direction =
                            MIRROR_REFLECTIONS[powered as usize].opposite_if(!direction);
                        self.power(&mut working_board, pos, powered);
                        self.power(&mut working_board, pos, direction);

                        if self.source_gone(pos, powered) {
                            let splitter = working_board.get_mut(pos).splitter_mut();
                            splitter.set(powered, false);
                        }
                    }
                }
                // Galvos change the rotation of the mirror they are
                // pointing into when powered by a beam.
                BeamTile::Galvo { direction, powered } => {
                    self.track_powered(working_board.get_mut(pos).directions_mut(), pos);

                    let BeamTile::Mirror {
                        galvoed,
                        powered: powered_sides,
                        ..
                    } = working_board.get_mut(direction.offset(pos))
                    else {
                        continue;
                    };

                    let opp_dir = direction.opposite();
                    let new = powered.any_but(opp_dir);

                    let changed = galvoed.contains(opp_dir) != new;
                    galvoed.set(opp_dir, new);

                    if changed {
                        // false => horizontal, true => vertical
                        let top_direction =
                            matches!(powered_sides[0], Some(Direction::Up | Direction::Down));
                        let bottom_direction =
                            matches!(powered_sides[1], Some(Direction::Up | Direction::Down));

                        if top_direction && !bottom_direction {
                            powered_sides[1] = None;
                        } else if bottom_direction && !top_direction {
                            powered_sides[0] = None;
                        } else if !top_direction || !bottom_direction {
                            powered_sides.swap(0, 1);
                        }
                    }
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
                galvoed,
                powered,
                direction: mirror_direction,
            } => {
                if *mirror_direction ^ galvoed.any() {
                    powered[matches!(direction, Direction::Up | Direction::Right) as usize] =
                        Some(direction);
                } else {
                    powered[matches!(direction, Direction::Up | Direction::Left) as usize] =
                        Some(direction);
                }
            }
            BeamTile::Splitter { powered, .. } => *powered |= direction,
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
