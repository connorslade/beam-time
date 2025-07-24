use common::direction::{Direction, Directions};

use super::MIRROR_REFLECTIONS;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BeamTile {
    #[default]
    Empty,
    Wall {
        powered: Directions,
    },
    Beam {
        direction: Direction,
        distance: u8,
    },
    CrossBeam {
        /// Directions of the two incoming beams.
        directions: [Direction; 2],
    },
    Emitter {
        direction: Direction,
        active: bool,
    },
    Detector {
        powered: Directions,
    },
    Delay {
        powered: Directions,
        last_powered: Directions,
    },
    Mirror {
        /// The direction the mirror is facing.
        galvoed: Directions,
        /// The direction the mirror was placed in.
        /// `0 => /`, `1 => \`
        direction: bool,
        /// Which direction the beam is coming from for each side.
        powered: [Option<Direction>; 2],
    },
    Splitter {
        direction: bool,
        powered: Option<Direction>,
    },
    Galvo {
        direction: Direction,
        powered: Directions,
    },
}

impl BeamTile {
    /// Checks if a tile is powered. This should be more efficient than
    /// power_direction, which only needs to be called if the tile is powered.
    pub fn is_powered(&self) -> bool {
        match self {
            Self::Emitter { active: true, .. } | Self::Beam { .. } | Self::CrossBeam { .. } => true,
            Self::Detector { powered } | Self::Galvo { powered, .. } | Self::Wall { powered } => {
                powered.any()
            }
            Self::Delay { last_powered, .. } => last_powered.any(),
            Self::Mirror { powered, .. } => powered[0].is_some() || powered[1].is_some(),
            Self::Splitter { powered, .. } => powered.is_some(),
            _ => false,
        }
    }

    /// Returns the directions of power output from a tile.
    pub fn power_direction(&self) -> Directions {
        match self {
            Self::Beam { direction, .. }
            | Self::Emitter {
                direction,
                active: true,
                ..
            } => direction.into(),
            Self::CrossBeam { directions } => directions.iter().copied().collect(),
            Self::Mirror {
                galvoed,
                powered,
                direction,
            } => powered
                .iter()
                .flatten()
                .map(|&powered| {
                    MIRROR_REFLECTIONS[powered as usize]
                        .opposite_if(!(direction ^ galvoed.odd_count()))
                })
                .collect(),
            Self::Splitter {
                direction,
                powered: Some(powered),
            } => {
                Directions::from(MIRROR_REFLECTIONS[*powered as usize].opposite_if(!*direction))
                    | *powered
            }
            Self::Delay { last_powered, .. } => *last_powered,
            _ => Directions::empty(),
        }
    }

    pub fn mirror_mut(&mut self) -> (bool, &mut Directions, &mut [Option<Direction>; 2]) {
        match self {
            Self::Mirror {
                galvoed,
                direction: original_direction,
                powered,
                ..
            } => (*original_direction, galvoed, powered),
            _ => panic!(),
        }
    }

    pub fn splitter_mut(&mut self) -> &mut Option<Direction> {
        match self {
            Self::Splitter { powered, .. } => powered,
            _ => panic!(),
        }
    }

    pub fn delay_mut(&mut self) -> (&mut Directions, &mut Directions) {
        match self {
            Self::Delay {
                powered,
                last_powered,
            } => (powered, last_powered),
            _ => panic!(),
        }
    }

    pub fn directions_mut(&mut self) -> &mut Directions {
        match self {
            Self::Galvo { powered, .. } | Self::Wall { powered } | Self::Detector { powered } => {
                powered
            }
            _ => panic!(),
        }
    }
}
