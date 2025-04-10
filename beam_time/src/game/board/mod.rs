use std::{
    fs::{self, File},
    path::PathBuf,
    time::Instant,
};

use anyhow::Result;
use bincode::Options;
use chrono::{DateTime, Utc};
use engine::exports::nalgebra::Vector2;
use log::{info, trace, warn};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::consts::AUTOSAVE_INTERVAL;
use beam_logic::{level::Level, tile::Tile};
use common::{consts::BINCODE_OPTIONS, map::Map};

use super::{history::History, holding::Holding, selection::SelectionState};

mod upgrade;

pub const SAVE_VERSION: u32 = 4;

#[derive(Default, Serialize, Deserialize)]
pub struct Board {
    pub meta: BoardMeta,
    pub notes: Vec<Note>,
    pub tiles: Map<Tile>,

    #[serde(skip)]
    pub transient: TransientBoardState,
}

pub struct TransientBoardState {
    pub holding: Holding,
    pub history: History,
    pub level: Option<&'static Level>,

    pub save_path: Option<PathBuf>,
    pub selection: SelectionState,

    pub open_timestamp: Instant,
    last_save: Instant,
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

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Note {
    position: Vector2<f32>,
    title: String,
    body: String,
}

impl Board {
    pub fn new_sandbox(name: String) -> Self {
        Self {
            meta: BoardMeta {
                version: SAVE_VERSION,
                name,
                level: None,
                size: None,
                last_played: Utc::now(),
                playtime: 0,
            },
            notes: Vec::new(),
            tiles: Map::default(),
            transient: TransientBoardState::default(),
        }
    }

    pub fn load(path: &PathBuf) -> Result<Self> {
        info!("Loading board from {path:?}");

        let file = File::open(path)?;
        let mut board = upgrade::load(file)?;
        board.transient.save_path = Some(path.to_path_buf());

        trace!("{:?}", board.meta);
        Ok(board)
    }

    pub fn load_meta(path: &PathBuf) -> Result<BoardMeta> {
        let file = File::open(path)?;
        let meta = upgrade::load_meta(file)?;
        Ok(meta)
    }

    pub fn save(mut self, path: &PathBuf) -> Result<()> {
        self.meta.playtime += self.transient.open_timestamp.elapsed().as_secs();
        self.meta.last_played = Utc::now();
        self.meta.version = SAVE_VERSION;

        let start = Instant::now();
        info!("Saving board to {path:?}");
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let file = File::create(path)?;
        BINCODE_OPTIONS.serialize_into(file, &self)?;

        info!("Save took {:?}", start.elapsed());
        Ok(())
    }
}

impl Board {
    pub fn is_permanent(&self, pos: &Vector2<i32>) -> bool {
        self.transient.level.map(|x| x.permanent.contains(pos)) == Some(true)
    }

    pub fn tick_autosave(&mut self) {
        if let Some(path) = &self.transient.save_path {
            if self.transient.last_save.elapsed() >= AUTOSAVE_INTERVAL {
                trace!("Autosaving...");
                self.transient.last_save = Instant::now();
                // run async if causing issues
                if let Err(err) = self.clone().save(path) {
                    warn!("Autosave failure: {err}");
                }
            }
        }
    }
}

impl BoardMeta {
    pub fn is_solved(&self) -> bool {
        self.level.map(|x| x.solved) == Some(true)
    }
}

impl Default for TransientBoardState {
    fn default() -> Self {
        Self {
            holding: Default::default(),
            history: History::new(),
            level: None,

            save_path: None,
            selection: Default::default(),

            open_timestamp: Instant::now(),
            last_save: Instant::now(),
        }
    }
}

impl Clone for Board {
    fn clone(&self) -> Self {
        Self {
            meta: self.meta.clone(),
            notes: self.notes.clone(),
            tiles: self.tiles.clone(),
            transient: TransientBoardState::default(),
        }
    }
}
