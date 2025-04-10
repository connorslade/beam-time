use std::io::{Read, Seek, SeekFrom};

use anyhow::{bail, Result};
use bincode::Options;
use log::info;
use serde::Deserialize;

use beam_logic::tile::Tile;
use common::{consts::BINCODE_OPTIONS, map::Map};

use super::{Board, BoardMeta, SAVE_VERSION};

pub fn load<R: Read + Seek>(mut data: R) -> Result<Board> {
    let version = BINCODE_OPTIONS.deserialize_from::<_, u32>(&mut data)?;
    data.seek(SeekFrom::Start(0))?;

    let board = versions!(version, data, [
        3 => version_3::Board,
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
        3 => super::BoardMeta,
        SAVE_VERSION => super::BoardMeta
    ]);

    Ok(meta)
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

mod version_3 {
    use super::*;

    #[derive(Deserialize)]
    pub struct Board {
        meta: super::BoardMeta,
        tiles: Map<Tile>,
    }

    impl From<Board> for super::Board {
        fn from(value: Board) -> Self {
            super::Board {
                meta: value.meta,
                tiles: value.tiles,
                ..Default::default()
            }
        }
    }
}
