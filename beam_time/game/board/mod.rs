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
#[cfg(feature = "steam")]
use crate::{app::App, game::achievements::award_sandbox_playtime_achievements};
use beam_logic::{level::Level, tile::Tile};
use common::{consts::BINCODE_OPTIONS, map::Map};

use super::{history::History, holding::Holding, selection::SelectionState};

pub mod unloaded;
mod upgrade;

pub const SAVE_VERSION: u32 = 6;

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
    pub deleting: bool,

    pub open_timestamp: Instant,
    pub trash: bool,
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

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LevelMeta {
    pub id: Uuid,
    pub solved: Option<LevelStats>,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LevelStats {
    pub cost: u32,
    pub latency: u32,
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Note {
    pub position: Vector2<f32>,
    pub title: String,
    pub body: String,
}

impl Board {
    pub fn new_sandbox(name: String) -> Self {
        Self {
            meta: BoardMeta {
                version: SAVE_VERSION,
                last_played: Utc::now(),
                name,
                ..Default::default()
            },
            ..Default::default()
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

        self.save_exact(path)?;
        Ok(())
    }

    /// Saves the board without editing the playtime or last_played timestamp.
    pub fn save_exact(mut self, path: &PathBuf) -> Result<()> {
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

    pub fn total_playtime(&self) -> u64 {
        self.meta.playtime + self.transient.open_timestamp.elapsed().as_secs()
    }

    pub fn tick_autosave(&mut self, #[cfg(feature = "steam")] app: &App) {
        if let Some(path) = &self.transient.save_path
            && self.transient.last_save.elapsed() >= AUTOSAVE_INTERVAL
        {
            #[cfg(feature = "steam")]
            (self.transient.level.is_some())
                .then(|| award_sandbox_playtime_achievements(app, self.total_playtime()));

            trace!("Autosaving...");
            self.transient.last_save = Instant::now();
            // run async if causing issues
            if let Err(err) = self.clone().save(path) {
                warn!("Autosave failure: {err}");
            }
        }
    }

    pub fn reset(&mut self) {
        self.notes.clear();
        self.tiles = Map::default();

        if let (Some(meta), Some(level)) = (&mut self.meta.level, self.transient.level) {
            meta.solved = None;
            self.tiles = level.tiles.clone();
        };
    }

    pub fn tile_props(&self, tile: &Tile, pos: &Vector2<i32>) -> (bool, bool, bool) {
        (tile.is_empty(), self.is_permanent(pos), tile.id().is_some())
    }

    pub fn in_bounds(&self, pos: &Vector2<i32>) -> bool {
        !(self.meta.size)
            .map(|size| pos.x < 0 || pos.y < 0 || pos.x as u32 >= size.x || pos.y as u32 >= size.y)
            .unwrap_or_default()
    }
}

impl BoardMeta {
    pub fn is_solved(&self) -> bool {
        self.level
            .as_ref()
            .map(|x| x.solved.is_some())
            .unwrap_or_default()
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
            deleting: false,

            open_timestamp: Instant::now(),
            trash: false,
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
