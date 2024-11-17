use std::{collections::HashSet, fs::File, path::PathBuf};

use anyhow::Result;
use engine::exports::nalgebra::Vector2;
use log::warn;
use once_cell::sync::Lazy;
use serde::Deserialize;
use uuid::Uuid;

use crate::misc::map::Map;

use super::tile::Tile;

pub macro default_level {
    ($name:expr) => {
        Level::load_slice(include_bytes!(concat!("../../assets/levels/", $name)))
    },
    ($($name:expr),*) => {{
        let mut out = Vec::new();
        $(
            match default_level!($name) {
                Ok(x) => out.push(x),
                Err(err) => warn!("Error loading level `{}`: {err}", $name)
            };
        )*
        return out;
    }}
}

pub static LEVELS: Lazy<Vec<Level>> = Lazy::new(|| {
    default_level!(
        "basic_routing.ron",
        "slightly_less_basic_routing.ron",
        "not_gate.ron",
        "and_gate.ron",
        "or_gate.ron",
        "basic_oscillator.ron",
        "synchronization.ron",
        "two_way_multiplexer.ron",
        "xor_gate.ron",
        "half_adder.ron",
        "two_tick_clock.ron",
        "even_oscillators.ron",
        // "rs_latch.ron",
        // "gated_d_latch.ron",
        // "double_it.ron",
        "count_ones.ron"
    )
});

#[derive(Debug, Deserialize)]
pub struct Level {
    pub id: Uuid,
    pub name: String,
    pub description: String,

    pub size: Option<Vector2<u32>>,
    pub permanent: HashSet<Vector2<i32>>,
    pub tiles: Map<Tile>,

    pub tests: Tests,
}

#[derive(Debug, Deserialize)]
pub struct Tests {
    pub cases: Vec<TestCase>,

    pub lasers: Vec<ElementLocation>,
    pub detectors: Vec<ElementLocation>,
}
#[derive(Debug, Deserialize)]
pub struct TestCase {
    pub lasers: Vec<bool>,
    pub detectors: Vec<Vec<bool>>,
}

#[derive(Debug, Clone, Copy, Deserialize)]
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

impl ElementLocation {
    pub fn into_pos(self) -> Vector2<i32> {
        match self {
            ElementLocation::Static(pos) => pos,
            ElementLocation::Dynamic(_) => todo!(),
        }
    }
}
