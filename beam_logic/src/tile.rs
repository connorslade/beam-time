use serde::{Deserialize, Serialize};

use common::direction::Direction;

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
    Wall,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
pub enum TileType {
    Detector,
    Delay,
    Emitter,
    Mirror,
    Splitter,
    Galvo,
    Wall,
}

impl Tile {
    pub const DEFAULT: [Tile; 7] = [
        Tile::Mirror { rotation: false },
        Tile::Splitter { rotation: false },
        Tile::Galvo {
            rotation: Direction::Up,
        },
        Tile::Emitter {
            rotation: Direction::Up,
            active: true,
        },
        Tile::Delay,
        Tile::Wall,
        Tile::Detector,
    ];

    pub fn is_empty(&self) -> bool {
        matches!(self, Tile::Empty)
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

    pub fn price(&self) -> u32 {
        match self {
            Tile::Empty => unreachable!(),
            Tile::Detector => 5000,
            Tile::Emitter { .. } => 1000,
            Tile::Delay => 500,
            Tile::Mirror { .. } => 200,
            Tile::Splitter { .. } => 300,
            Tile::Galvo { .. } => 500,
            Tile::Wall { .. } => 100,
        }
    }

    pub fn as_type(&self) -> TileType {
        match self {
            Tile::Empty => unreachable!(),
            Tile::Detector => TileType::Detector,
            Tile::Delay => TileType::Delay,
            Tile::Emitter { .. } => TileType::Emitter,
            Tile::Mirror { .. } => TileType::Mirror,
            Tile::Splitter { .. } => TileType::Splitter,
            Tile::Galvo { .. } => TileType::Galvo,
            Tile::Wall => TileType::Wall,
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
                rotation: rotation.rotate_reverse(),
                active,
            },
            Tile::Galvo { rotation } => Tile::Galvo {
                rotation: rotation.rotate_reverse(),
            },
            x => x.rotate(),
        }
    }

    pub fn flip_horizontal(self) -> Self {
        match self {
            Tile::Emitter { rotation, active } => Tile::Emitter {
                rotation: rotation.flip_horizontal(),
                active,
            },
            Tile::Galvo { rotation } => Tile::Galvo {
                rotation: rotation.flip_horizontal(),
            },
            Tile::Mirror { .. } | Tile::Splitter { .. } => self.rotate(),
            x => x,
        }
    }

    pub fn flip_vertical(self) -> Self {
        match self {
            Tile::Emitter { rotation, active } => Tile::Emitter {
                rotation: rotation.flip_vertical(),
                active,
            },
            Tile::Galvo { rotation } => Tile::Galvo {
                rotation: rotation.flip_vertical(),
            },
            Tile::Mirror { .. } | Tile::Splitter { .. } => self.rotate(),
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
