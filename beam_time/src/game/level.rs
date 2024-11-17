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
        "1_basic_routing.ron",
        "2_slightly_less_basic_routing.ron",
        "3_not_gate.ron",
        "4_and_gate.ron",
        "5_or_gate.ron",
        "6_basic_oscillator.ron",
        "7_synchronization.ron",
        "8_two_way_multiplexer.ron",
        "9_xor_gate.ron",
        "10_half_adder.ron",
        "11_two_tick_clock.ron",
        "12_even_oscillators.ron",
        // "13_rs_latch.ron",
        // "14_gated_d_latch.ron",
        // "15_double_it.ron",
        "16_count_ones.ron"
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
