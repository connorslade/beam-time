use std::io::{Read, Seek, SeekFrom};

use anyhow::{Result, bail};
use bincode::Options;
use log::info;
use serde::Deserialize;

use beam_logic::tile::Tile;
use common::{consts::BINCODE_OPTIONS, map::Map};

use super::{Board, BoardMeta, LevelMeta, SAVE_VERSION};

pub fn load<R: Read + Seek>(mut data: R) -> Result<Board> {
    let version = BINCODE_OPTIONS.deserialize_from::<_, u32>(&mut data)?;
    data.seek(SeekFrom::Start(0))?;

    let board = versions!(version, data, [
        3 => version_3::Board,
        4 => version_4::Board,
        5 => version_5::Board,
        SAVE_VERSION => super::Board
    ]);

    if version != SAVE_VERSION {
        info!("Upgraded save from version {version} to {SAVE_VERSION}");
    }

    Ok(board)
}

pub fn load_meta<R: Read + Seek>(mut data: R) -> Result<BoardMeta> {
    let version = BINCODE_OPTIONS.deserialize_from::<_, u32>(&mut data)?;
    data.seek(SeekFrom::Start(0))?;

    let meta = versions!(version, data, [
        3..=5 => version_5::BoardMeta,
        SAVE_VERSION => super::BoardMeta
    ]);

    Ok(meta)
}

mod version_3 {
    use common::direction::Direction;

    use super::*;

    #[derive(Deserialize)]
    pub struct Board {
        meta: version_5::BoardMeta,
        tiles: Map<Tile>,
    }

    #[derive(Default, Copy, Clone, PartialEq, Eq, Deserialize)]
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

    impl From<Tile> for super::Tile {
        fn from(value: Tile) -> Self {
            match value {
                Tile::Empty => super::Tile::Empty,
                Tile::Detector => super::Tile::Detector { id: None },
                Tile::Delay => super::Tile::Delay,
                Tile::Emitter { rotation, active } => super::Tile::Emitter {
                    rotation,
                    active,
                    id: None,
                },
                Tile::Mirror { rotation } => super::Tile::Mirror { rotation },
                Tile::Splitter { rotation } => super::Tile::Splitter { rotation },
                Tile::Galvo { rotation } => super::Tile::Galvo { rotation },
                Tile::Wall => super::Tile::Wall,
            }
        }
    }

    impl From<Board> for super::Board {
        fn from(value: Board) -> Self {
            super::Board {
                meta: value.meta.into(),
                tiles: value.tiles.map(|_, x| x.into()),
                ..Default::default()
            }
        }
    }
}

mod version_4 {
    use crate::game::board::Note;

    use super::*;

    #[derive(Deserialize)]
    pub struct Board {
        meta: version_5::BoardMeta,
        notes: Vec<Note>,
        tiles: Map<version_3::Tile>,
    }

    impl From<Board> for super::Board {
        fn from(value: Board) -> Self {
            super::Board {
                meta: value.meta.into(),
                tiles: value.tiles.map(|_, x| x.into()),
                notes: value.notes,
                ..Default::default()
            }
        }
    }
}

mod version_5 {
    use chrono::{DateTime, Utc};
    use engine::exports::nalgebra::Vector2;
    use serde::Serialize;
    use uuid::Uuid;

    use super::*;
    use crate::game::board::Note;

    #[derive(Deserialize)]
    pub struct Board {
        meta: BoardMeta,
        notes: Vec<Note>,
        tiles: Map<Tile>,
    }

    #[derive(Default, Clone, Debug, Serialize, Deserialize)]
    pub struct BoardMeta {
        pub version: u32,

        pub name: String,
        pub level: Option<LevelMeta>,
        pub size: Option<Vector2<u32>>,

        pub last_played: DateTime<Utc>,
        pub playtime: u64,
    }

    #[derive(Default, Clone, Copy, Debug, Serialize, Deserialize)]
    pub struct LevelMeta {
        pub id: Uuid,
        pub solved: bool,
    }

    impl From<Board> for super::Board {
        fn from(value: Board) -> Self {
            Self {
                meta: value.meta.into(),
                notes: value.notes,
                tiles: value.tiles,
                ..Default::default()
            }
        }
    }

    impl From<BoardMeta> for super::BoardMeta {
        fn from(value: BoardMeta) -> Self {
            super::BoardMeta {
                version: value.version,
                name: value.name,
                level: value.level.map(|x| x.into()),
                size: value.size,
                last_played: value.last_played,
                playtime: value.playtime,
            }
        }
    }

    impl From<LevelMeta> for super::LevelMeta {
        fn from(value: LevelMeta) -> Self {
            Self {
                id: value.id,
                name: "New Solution 1".into(),
                solved: None,
            }
        }
    }
}

macro versions($ver:expr, $data:expr, [$($version:pat => $module:ty),*]) {
    match $ver {
        $(
            $version => common::consts::BINCODE_OPTIONS
                .deserialize_from::<_, $module>(&mut $data)?
                .into(),
        )*
        _ => bail!("Unknown save version `{}`", $ver),
    }
}
