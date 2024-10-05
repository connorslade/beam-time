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

impl BeamTile {
    pub fn is_powered(&self) -> bool {
        match self {
            Self::Emitter { .. } | Self::Beam { .. } => true,
            Self::Mirror { powered, .. } => powered[0].is_some() || powered[1].is_some(),
            _ => false,
        }
    }

    pub fn power_direction(&self) -> Directions {
        match self {
            Self::Beam { direction } | Self::Emitter { direction } => direction.into(),
            Self::Mirror { direction, powered } => powered
                .iter()
                .flatten()
                .map(|&powered| opposite_if(MIRROR_REFLECTIONS[powered as usize], !direction))
                .collect(),
            _ => Directions::empty(),
        }
    }
}
