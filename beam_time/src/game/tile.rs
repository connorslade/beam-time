use engine::assets::SpriteRef;
use serde::{Deserialize, Serialize};

use crate::{
    assets::{TILE_DELAY, TILE_DETECTOR, TILE_WALL},
    consts::{EMITTER, GALVO, MIRROR, SPLITTER},
    misc::direction::Direction,
};

#[derive(Default, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Tile {
    #[default]
    Empty,
    Detector,
    Delay,
    Emitter {
        rotation: Direction,
        active: bool,
    },
    Mirror {
        rotation: bool,
    },
    Splitter {
        rotation: bool,
    },
    Galvo {
        rotation: Direction,
    },
    Wall {
        permanent: bool,
    },
}

impl Tile {
    pub const DEFAULT: [Tile; 7] = [
        Tile::Emitter {
            rotation: Direction::Up,
            active: true,
        },
        Tile::Galvo {
            rotation: Direction::Up,
        },
        Tile::Splitter { rotation: false },
        Tile::Mirror { rotation: false },
        Tile::Delay,
        Tile::Wall { permanent: false },
        Tile::Detector,
    ];

    pub fn is_empty(&self) -> bool {
        matches!(self, Tile::Empty)
    }

    pub fn moveable(&self) -> bool {
        !matches!(self, Tile::Wall { permanent: true })
    }

    pub fn permanent(&self) -> bool {
        matches!(self, Tile::Wall { permanent: true })
    }

    pub fn name(&self) -> &str {
        match self {
            Tile::Empty => unreachable!(),
            Tile::Detector => "Detector",
            Tile::Emitter { .. } => "Emitter",
            Tile::Delay => "Delay",
            Tile::Mirror { .. } => "Mirror",
            Tile::Splitter { .. } => "Splitter",
            Tile::Galvo { .. } => "Galvo",
            Tile::Wall { .. } => "Wall",
        }
    }

    pub fn asset(&self) -> SpriteRef {
        match self {
            Tile::Empty => unreachable!(),
            Tile::Detector => TILE_DETECTOR,
            Tile::Delay => TILE_DELAY,
            Tile::Emitter { rotation, .. } => EMITTER[*rotation as usize],
            Tile::Mirror { rotation, .. } => MIRROR[*rotation as usize],
            Tile::Splitter { rotation, .. } => SPLITTER[*rotation as usize],
            Tile::Galvo { rotation, .. } => GALVO[*rotation as usize],
            Tile::Wall { .. } => TILE_WALL,
        }
    }

    pub fn rotate(self) -> Self {
        match self {
            Tile::Emitter { rotation, active } => Tile::Emitter {
                rotation: rotation.rotate(),
                active,
            },
            Tile::Mirror { rotation } => Tile::Mirror {
                rotation: !rotation,
            },
            Tile::Splitter { rotation } => Tile::Splitter {
                rotation: !rotation,
            },
            Tile::Galvo { rotation } => Tile::Galvo {
                rotation: rotation.rotate(),
            },
            x => x,
        }
    }

    pub fn rotate_reverse(self) -> Self {
        match self {
            Tile::Emitter { rotation, active } => Tile::Emitter {
                rotation: rotation.roate_reverse(),
                active,
            },
            Tile::Galvo { rotation } => Tile::Galvo {
                rotation: rotation.roate_reverse(),
            },
            x => x.rotate(),
        }
    }

    pub fn activate(self) -> Self {
        match self {
            Self::Emitter { rotation, active } => Self::Emitter {
                rotation,
                active: !active,
            },
            x => x,
        }
    }
}
