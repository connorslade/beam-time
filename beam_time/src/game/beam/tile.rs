use engine::drawable::sprite::Sprite;

use crate::{
    assets::{animated_sprite, TILE_DETECTOR, TILE_MIRROR_A, TILE_MIRROR_B},
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
    Mirror {
        direction: bool,
        original_direction: bool,
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
            _ => false,
        }
    }

    /// Returns the directions of power output from a tile.
    pub fn power_direction(&self) -> Directions {
        match self {
            Self::Beam { direction }
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
            _ => Directions::empty(),
        }
    }

    // todo: don't return options
    pub fn emitter_mut(&mut self) -> Option<&mut bool> {
        match self {
            Self::Emitter { active, .. } => Some(active),
            _ => None,
        }
    }

    pub fn mirror_mut(&mut self) -> Option<(bool, &mut bool, &mut [Option<Direction>; 2])> {
        match self {
            Self::Mirror {
                direction,
                original_direction,
                powered,
                ..
            } => Some((*original_direction, direction, powered)),
            _ => None,
        }
    }

    pub fn splitter_mut(&mut self) -> Option<&mut Option<Direction>> {
        match self {
            Self::Splitter { powered, .. } => Some(powered),
            _ => None,
        }
    }

    pub fn directions_mut(&mut self) -> Option<&mut Directions> {
        match self {
            Self::Galvo { powered, .. } | Self::Wall { powered } | Self::Detector { powered } => {
                Some(powered)
            }
            _ => None,
        }
    }
}
