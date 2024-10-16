use engine::drawable::sprite::Sprite;

use crate::{
    assets::{animated_sprite, TILE_DELAY, TILE_DETECTOR, TILE_MIRROR_A, TILE_MIRROR_B},
    consts::{EMITTER, GALVO, SPLITTER},
    misc::direction::{Direction, Directions},
};

use super::{opposite_if, MIRROR_REFLECTIONS};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
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
        /// `0 => /`, `1 => \`
        direction: bool,
        /// The direction the mirror was placed in.
        original_direction: bool,
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
    /// Overwrites the texture of a tile for rendering purposes.
    pub fn base_sprite(&self, frame: u8) -> Option<Sprite> {
        Some(match self {
            BeamTile::Emitter { direction, active } => {
                animated_sprite(EMITTER[*direction as usize], *active, frame)
            }
            BeamTile::Detector { powered } => animated_sprite(TILE_DETECTOR, powered.any(), frame),
            BeamTile::Delay { powered, .. } => animated_sprite(TILE_DELAY, powered.any(), frame),
            BeamTile::Mirror {
                direction,
                original_direction,
                ..
            } => animated_sprite(
                [TILE_MIRROR_A, TILE_MIRROR_B][*direction as usize],
                direction != original_direction,
                frame,
            ),
            BeamTile::Galvo { direction, powered } if powered.any_but(direction.opposite()) => {
                animated_sprite(GALVO[*direction as usize], true, frame)
            }
            BeamTile::Splitter {
                direction,
                powered: Some(..),
            } => animated_sprite(SPLITTER[*direction as usize], true, frame),
            _ => return None,
        })
    }

    /// Checks if a tile is powered. This should be more efficient than
    /// power_direction, which only needs to be called if the tile is powered.
    pub fn is_powered(&self) -> bool {
        match self {
            Self::Emitter { active: true, .. } | Self::Beam { .. } | Self::CrossBeam { .. } => true,
            Self::Mirror { powered, .. } => powered[0].is_some() || powered[1].is_some(),
            Self::Splitter { powered, .. } => powered.is_some(),
            Self::Delay { last_powered, .. } => last_powered.any(),
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
                direction, powered, ..
            } => powered
                .iter()
                .flatten()
                .map(|&powered| opposite_if(MIRROR_REFLECTIONS[powered as usize], !direction))
                .collect(),
            Self::Splitter {
                direction,
                powered: Some(powered),
            } => {
                Directions::from(opposite_if(
                    MIRROR_REFLECTIONS[*powered as usize],
                    !*direction,
                )) | *powered
            }
            Self::Delay { last_powered, .. } => *last_powered,
            _ => Directions::empty(),
        }
    }

    pub fn mirror_mut(&mut self) -> (bool, &mut bool, &mut [Option<Direction>; 2]) {
        match self {
            Self::Mirror {
                direction,
                original_direction,
                powered,
                ..
            } => (*original_direction, direction, powered),
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
