use engine::assets::SpriteRef;

use crate::assets::{
    GALVO_TILE, MIRROR_A_TILE, MIRROR_B_TILE, SPLITTER_A_TILE, SPLITTER_B_TILE, WALL_TILE,
};

#[derive(Copy, Clone)]
pub enum Tile {
    Empty,
    Mirror { rotation: u8 },
    Splitter { rotation: u8 },
    Galvo,
    Wall,
}

impl Tile {
    pub const DEFAULT: [Tile; 3] = [
        Tile::Mirror { rotation: 0 },
        Tile::Splitter { rotation: 0 },
        Tile::Galvo,
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
            Tile::Mirror { .. } => "Mirror",
            Tile::Splitter { .. } => "Prism",
            Tile::Galvo => "Galvo",
            Tile::Wall => "Wall",
        }
    }

    pub fn asset(&self) -> SpriteRef {
        match self {
            Tile::Empty => unreachable!(),
            Tile::Mirror { rotation } => match rotation {
                0 => MIRROR_A_TILE,
                1 => MIRROR_B_TILE,
                _ => unreachable!(),
            },
            Tile::Splitter { rotation } => match rotation {
                0 => SPLITTER_A_TILE,
                1 => SPLITTER_B_TILE,
                _ => unreachable!(),
            },
            Tile::Galvo => GALVO_TILE,
            Tile::Wall => WALL_TILE,
        }
    }

    pub fn rotate(self) -> Self {
        match self {
            Tile::Empty => Tile::Empty,
            Tile::Mirror { rotation } => Tile::Mirror {
                rotation: (rotation + 1) % 2,
            },
            Tile::Splitter { rotation } => Tile::Splitter {
                rotation: (rotation + 1) % 2,
            },
            Tile::Galvo => Tile::Galvo,
            Tile::Wall => Tile::Wall,
        }
    }
}
