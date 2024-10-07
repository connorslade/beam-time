use std::f32::consts::PI;

use engine::assets::SpriteRef;

use crate::{
    assets::{
        ACTIVE_EMITTER_TILE, EMITTER_TILE, GALVO_TILE, MIRROR_TILE, SPLITTER_TILE, WALL_TILE,
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
            Tile::Emitter { active: false, .. } => EMITTER_TILE,
            Tile::Emitter { active: true, .. } => ACTIVE_EMITTER_TILE,
            Tile::Mirror { .. } => MIRROR_TILE,
            Tile::Splitter { .. } => SPLITTER_TILE,
            Tile::Galvo { .. } => GALVO_TILE,
            Tile::Wall { .. } => WALL_TILE,
        }
    }

    pub fn sprite_rotation(&self) -> f32 {
        match self {
            Tile::Emitter { rotation, .. } | Tile::Galvo { rotation } => rotation.to_angle(),
            Tile::Mirror { rotation: true } | Tile::Splitter { rotation: true } => PI / 2.0,
            _ => 0.0,
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
