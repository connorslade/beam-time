use std::{fs::File, path::PathBuf};

use ahash::{HashMap, HashMapExt, HashSet};
use anyhow::Result;
use common::map::Map;
use nalgebra::Vector2;
use ron::{Options, extensions::Extensions};
use serde::{Deserialize, Deserializer};
use uuid::Uuid;

use crate::tile::{Tile, TileType};

pub mod case;
pub mod default;
pub mod tree;
use case::TestCase;

#[derive(Debug, Clone, Deserialize)]
pub struct Level {
    /// The ID is stored in the save file, meaning you can share your campaign
    /// levels and other can view correctly them in sandbox mode.
    pub id: Uuid,
    pub name: String,
    #[serde(deserialize_with = "unindent_string")]
    pub description: String,

    pub parents: Vec<Uuid>,

    #[serde(default)]
    pub size: Option<Vector2<u32>>,
    #[serde(default)]
    pub permanent: HashSet<Vector2<i32>>,
    #[serde(default)]
    pub labels: HashMap<ElementLocation, String>,
    #[serde(default)]
    pub disabled: Option<HashSet<TileType>>,

    pub tiles: Map<Tile>,

    pub tests: Tests,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Tests {
    pub lasers: Vec<u32>,
    pub detectors: Vec<u32>,
    #[serde(default)]
    pub display: Option<DisplayConfig>,

    #[serde(default)]
    pub hidden: HashSet<u32>,
    pub cases: Vec<TestCase>,
}

#[derive(Debug, Default, Clone, Deserialize)]
#[serde(default)]
pub struct DisplayConfig {
    pub emitter_breaks: HashSet<u32>,
    pub emitter_spaces: HashSet<u32>,

    pub detector_breaks: HashSet<u32>,
    pub detector_spaces: HashSet<u32>,

    pub descriptions: HashMap<u32, String>,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Deserialize)]
pub enum ElementLocation {
    Static(Vector2<i32>),
    Dynamic(u32),
}

#[derive(Default)]
pub struct DynamicElementMap {
    inner: HashMap<u32, Vector2<i32>>,
}

#[derive(Clone, Copy)]
pub enum LevelIo {
    Emitter,
    Detector,
}

impl Level {
    pub fn load_file(path: PathBuf) -> Result<Self> {
        let file = File::open(path)?;
        let ron = Options::default().with_default_extension(Extensions::IMPLICIT_SOME);
        Ok(ron.from_reader(file)?)
    }

    pub fn load_slice(slice: &[u8]) -> Result<Self> {
        let ron = Options::default().with_default_extension(Extensions::IMPLICIT_SOME);
        Ok(ron.from_bytes(slice)?)
    }

    pub fn is_dynamic(&self, id: u32) -> bool {
        self.tests.detectors.contains(&id) || self.tests.lasers.contains(&id)
    }
}

impl DynamicElementMap {
    pub fn position(&self, id: u32) -> Option<Vector2<i32>> {
        self.inner.get(&id).copied()
    }

    pub fn from_map(map: &Map<Tile>) -> Self {
        let mut inner = HashMap::new();
        for (pos, tile) in map.iter() {
            tile.id().and_then(|id| inner.insert(id, pos));
        }

        Self { inner }
    }
}

impl Tests {
    pub fn visible_count(&self) -> usize {
        self.cases.len() - self.hidden.len()
    }

    pub fn get_visible(&self, idx: usize) -> &TestCase {
        let mut active = 0;

        for (i, case) in self.cases.iter().enumerate() {
            if !self.hidden.contains(&(i as u32)) {
                if active == idx {
                    return case;
                }

                active += 1;
            }
        }

        panic!()
    }
}

impl DisplayConfig {
    pub fn do_break(&self, io: LevelIo, idx: usize) -> bool {
        let idx = idx as u32;
        match io {
            LevelIo::Emitter => self.emitter_breaks.contains(&idx),
            LevelIo::Detector => self.detector_breaks.contains(&idx),
        }
    }

    pub fn do_space(&self, io: LevelIo, idx: usize) -> bool {
        let idx = idx as u32;
        match io {
            LevelIo::Emitter => self.emitter_spaces.contains(&idx),
            LevelIo::Detector => self.detector_spaces.contains(&idx),
        }
    }
}

fn unindent_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let string = String::deserialize(deserializer)?;
    Ok(unindent::unindent(&string))
}
