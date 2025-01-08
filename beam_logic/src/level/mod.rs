use std::{
    collections::{HashMap, HashSet},
    fs::File,
    path::PathBuf,
};

use anyhow::Result;
use common::map::Map;
use log::warn;
use nalgebra::Vector2;
use once_cell::sync::Lazy;
use ron::{extensions::Extensions, Options};
use serde::{Deserialize, Deserializer};
use uuid::Uuid;

use crate::tile::{Tile, TileType};

pub mod case;
use case::TestCase;

pub macro default_level {
    ($name:expr) => {
        Level::load_slice(include_bytes!(concat!("../../../beam_time/assets/levels/", $name)))
    },
    ($($name:expr),* $(,)?) => {{
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

pub static DEFAULT_LEVELS: Lazy<Vec<Level>> = Lazy::new(|| {
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
        "even_oscillators.ron",
        "two_tick_clock.ron",
        "bit_reverse.ron",
        "double_it.ron",
        "four_bit_not.ron",
        "edge_detectors.ron",
        "rs_latch.ron",
        "gated_d_latch.ron",
        "t_flip_flop.ron",
        "read_only_memory.ron",
        "counter.ron",
        "full_adder.ron",
        "count_ones.ron",
    )
});

#[derive(Debug, Clone, Deserialize)]
pub struct Level {
    /// The ID is stored in the save file, meaning you can share your campaign
    /// levels and other can view correctly them in sandbox mode.
    pub id: Uuid,
    pub name: String,
    #[serde(deserialize_with = "unindent_string")]
    pub description: String,

    pub size: Option<Vector2<u32>>,
    pub permanent: HashSet<Vector2<i32>>,
    pub labels: HashMap<Vector2<i32>, String>,
    pub disabled: Option<HashSet<TileType>>,

    pub tiles: Map<Tile>,

    pub tests: Tests,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Tests {
    pub lasers: Vec<ElementLocation>,
    pub detectors: Vec<ElementLocation>,

    pub cases: Vec<TestCase>,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub enum ElementLocation {
    Static(Vector2<i32>),
    Dynamic(usize),
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
}

impl ElementLocation {
    pub fn into_pos(self) -> Vector2<i32> {
        match self {
            ElementLocation::Static(pos) => pos,
            ElementLocation::Dynamic(_) => todo!(),
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
