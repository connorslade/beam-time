use std::f32::consts::PI;

use engine::assets::SpriteRef;

use crate::{
    assets::{EMITTER_TILE, GALVO_TILE, MIRROR_TILE, SPLITTER_TILE, WALL_TILE},
    misc::direction::Direction,
};

#[derive(Copy, Clone)]
pub enum Tile {
    Empty,
    Emitter { rotation: Direction },
    Mirror { rotation: bool },
    Splitter { rotation: bool },
    Galvo,
    Wall,
}

impl Tile {
    pub const DEFAULT: [Tile; 4] = [
        Tile::Mirror { rotation: false },
        Tile::Splitter { rotation: false },
        Tile::Galvo,
        Tile::Emitter {
            rotation: Direction::Right,
        },
    ];

    pub fn is_empty(&self) -> bool {
        matches!(self, Tile::Empty)
    }

    pub fn is_some(&self) -> bool {
        !self.is_empty()
    }

    pub fn moveable(&self) -> bool {
        !matches!(self, Tile::Wall)
    }

    pub fn name(&self) -> &str {
        match self {
            Tile::Empty => unreachable!(),
            Tile::Emitter { .. } => "Emitter",
            Tile::Mirror { .. } => "Mirror",
            Tile::Splitter { .. } => "Splitter",
            Tile::Galvo => "Galvo",
            Tile::Wall => "Wall",
        }
    }

    pub fn asset(&self) -> SpriteRef {
        match self {
            Tile::Empty => unreachable!(),
            Tile::Emitter { .. } => EMITTER_TILE,
            Tile::Mirror { .. } => MIRROR_TILE,
            Tile::Splitter { .. } => SPLITTER_TILE,
            Tile::Galvo => GALVO_TILE,
            Tile::Wall => WALL_TILE,
        }
    }

    pub fn sprite_rotation(&self) -> f32 {
        match self {
            Tile::Emitter { rotation } => rotation.to_angle(),
            Tile::Mirror { rotation: true } | Tile::Splitter { rotation: true } => PI / 2.0,
            _ => 0.0,
        }
    }

    pub fn rotate(self) -> Self {
        match self {
            Tile::Empty => Tile::Empty,
            Tile::Emitter { rotation } => Tile::Emitter {
                rotation: rotation.rotate(),
            },
            Tile::Mirror { rotation } => Tile::Mirror {
                rotation: !rotation,
            },
            Tile::Splitter { rotation } => Tile::Splitter {
                rotation: !rotation,
            },
            Tile::Galvo => Tile::Galvo,
            Tile::Wall => Tile::Wall,
        }
    }
}
