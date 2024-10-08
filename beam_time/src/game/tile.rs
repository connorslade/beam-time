use engine::assets::SpriteRef;

use crate::{
    assets::{
        EMITTER, GALVO, SPLITTER, TILE_MIRROR_A, TILE_MIRROR_B, TILE_SPLITTER_A, TILE_SPLITTER_B,
        TILE_WALL,
    },
    misc::direction::Direction,
};

#[derive(Copy, Clone)]
pub enum Tile {
    Empty,
    Emitter { rotation: Direction, active: bool },
    Mirror { rotation: bool },
    Splitter { rotation: bool },
    Galvo { rotation: Direction },
    Wall { permanent: bool },
}

impl Tile {
    pub const DEFAULT: [Tile; 5] = [
        Tile::Wall { permanent: false },
        Tile::Mirror { rotation: false },
        Tile::Splitter { rotation: false },
        Tile::Galvo {
            rotation: Direction::Up,
        },
        Tile::Emitter {
            rotation: Direction::Up,
            active: true,
        },
    ];

    pub fn is_empty(&self) -> bool {
        matches!(self, Tile::Empty)
    }

    pub fn is_some(&self) -> bool {
        !self.is_empty()
    }

    pub fn moveable(&self) -> bool {
        !matches!(self, Tile::Wall { permanent: true })
    }

    pub fn name(&self) -> &str {
        match self {
            Tile::Empty => unreachable!(),
            Tile::Emitter { .. } => "Emitter",
            Tile::Mirror { .. } => "Mirror",
            Tile::Splitter { .. } => "Splitter",
            Tile::Galvo { .. } => "Galvo",
            Tile::Wall { .. } => "Wall",
        }
    }

    pub fn asset(&self) -> SpriteRef {
        match self {
            Tile::Empty => unreachable!(),
            Tile::Emitter { rotation, .. } => EMITTER[*rotation as usize],
            Tile::Mirror {
                rotation: false, ..
            } => TILE_MIRROR_A,
            Tile::Mirror { rotation: true, .. } => TILE_MIRROR_B,
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
