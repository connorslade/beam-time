use std::f32::consts::PI;

use crate::misc::direction::{Direction, Directions};

use super::{opposite_if, MIRROR_REFLECTIONS};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum BeamTile {
    #[default]
    Empty,
    Wall,
    Beam {
        direction: Direction,
    },
    CrossBeam {
        /// Directions of the two incoming beams.
        directions: [Direction; 2],
    },
    Emitter {
        direction: Direction,
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
        powered: Option<Direction>,
    },
}

impl BeamTile {
    /// Overwrites the rotation of a tile for rendering purposes. This is mainly
    /// used for mirrors, which can be rotated by galvos, and that needs to be
    /// reflected in the rendering.
    pub fn tile_rotation(&self) -> Option<f32> {
        match self {
            Self::Mirror { direction, .. } => Some(PI / 2.0 * *direction as u8 as f32),
            _ => None,
        }
    }

    /// Checks if a tile is powered. This should be more efficient than
    /// power_direction, which only needs to be called if the tile is powered.
    pub fn is_powered(&self) -> bool {
        match self {
            Self::Emitter { .. } | Self::Beam { .. } | Self::CrossBeam { .. } => true,
            Self::Mirror { powered, .. } => powered[0].is_some() || powered[1].is_some(),
            Self::Splitter { powered, .. } => powered.is_some(),
            _ => false,
        }
    }

    /// Returns the directions of power output from a tile.
    pub fn power_direction(&self) -> Directions {
        match self {
            Self::Beam { direction } | Self::Emitter { direction } => direction.into(),
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

    /// Returns a mutable reference to the inner data of a mirror tile.
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

    /// Returns a mutable reference to the inner data of a splitter tile.
    pub fn splitter_mut(&mut self) -> Option<&mut Option<Direction>> {
        match self {
            Self::Splitter { powered, .. } => Some(powered),
            _ => None,
        }
    }

    /// Returns a mutable reference to the inner data of a galvo tile.
    pub fn galvo_mut(&mut self) -> Option<&mut Option<Direction>> {
        match self {
            Self::Galvo { powered, .. } => Some(powered),
            _ => None,
        }
    }
}
