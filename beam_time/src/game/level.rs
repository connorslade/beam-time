use std::{fs::File, path::PathBuf};

use anyhow::Result;
use engine::exports::nalgebra::Vector2;
use once_cell::sync::Lazy;
use serde::Deserialize;
use uuid::Uuid;

use crate::misc::map::Map;

use super::tile::Tile;

pub static LEVELS: Lazy<Vec<Level>> = Lazy::new(|| {
    vec![Level::load_slice(include_bytes!("../../assets/levels/level_1.ron")).unwrap()]
});

#[derive(Debug, Deserialize)]
pub struct Level {
    pub id: Uuid,
    pub name: String,
    pub description: String,

    pub size: Option<Vector2<u32>>,
    pub tiles: Map<Tile>,

    pub tests: Tests,
}

#[derive(Debug, Deserialize)]
pub struct Tests {
    cases: Vec<TestCase>,

    lasers: Vec<ElementLocation>,
    detectors: Vec<ElementLocation>,
}
#[derive(Debug, Deserialize)]
pub struct TestCase {
    lasers: Vec<bool>,
    detectors: Vec<bool>,
}

#[derive(Debug, Deserialize)]
pub enum ElementLocation {
    Static(Vector2<i32>),
    Dynamic(usize),
}

impl Level {
    pub fn load_file(path: PathBuf) -> Result<Self> {
        let file = File::open(path)?;
        let level = ron::de::from_reader::<_, Self>(file)?;
        Ok(level)
    }

    pub fn load_slice(slice: &[u8]) -> Result<Self> {
        Ok(ron::de::from_bytes(slice)?)
    }
}
