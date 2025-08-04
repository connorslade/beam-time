use std::{fs::File, path::PathBuf};

use ahash::{HashMap, HashMapExt, HashSet};
use anyhow::Result;
use common::map::Map;
use log::warn;
use nalgebra::Vector2;
use once_cell::sync::Lazy;
use ron::{Options, extensions::Extensions};
use serde::{Deserialize, Deserializer};
use uuid::Uuid;

use crate::tile::{Tile, TileType};

pub mod case;
pub mod tree;
use case::TestCase;

pub macro default_level {
    ($name:expr) => {
        Level::load_slice(include_bytes!(concat!("../../assets/levels/", $name)))
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
        "accumulator.ron",
        "adder.ron",
        "adder_subtractor.ron",
        "and_gate.ron",
        "another_or_gate.ron",
        "barrel_shifter.ron",
        "basic_oscillator.ron",
        "basic_routing.ron",
        "bidirectional_counter.ron",
        "bit_reverse.ron",
        "conway_life.ron",
        "count_ones.ron",
        "counter.ron",
        "double_it.ron",
        "edge_detectors.ron",
        "even_oscillators.ron",
        "find_first_set.ron",
        "four_bit_not.ron",
        "full_adder.ron",
        "gated_d_latch.ron",
        "grey_decode.ron",
        "grey_encode.ron",
        "half_adder.ron",
        "hamming_correction.ron",
        "hamming_generation.ron",
        "imply_gate.ron",
        "large_multiplexer.ron",
        "multiplier.ron",
        "not_gate.ron",
        "one_tick_clock.ron",
        "or_gate.ron",
        "paralel_to_serial.ron",
        "parity_bit.ron",
        "program_counter.ron",
        "random_access_memory.ron",
        "read_only_memory.ron",
        "rs_latch.ron",
        "seven_segment_driver.ron",
        "shift_register.ron",
        "slightly_less_basic_routing.ron",
        "stack.ron",
        "synchronization.ron",
        "t_flip_flop.ron",
        "triple_it.ron",
        "two_tick_clock.ron",
        "two_way_multiplexer.ron",
        "twos_complement.ron",
        "xor_gate.ron",
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

    pub parents: Vec<Uuid>,

    pub size: Option<Vector2<u32>>,
    pub permanent: HashSet<Vector2<i32>>,
    pub labels: HashMap<ElementLocation, String>,
    pub disabled: Option<HashSet<TileType>>,

    pub tiles: Map<Tile>,

    pub tests: Tests,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Tests {
    pub lasers: Vec<u32>,
    pub detectors: Vec<u32>,

    pub cases: Vec<TestCase>,
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

fn unindent_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let string = String::deserialize(deserializer)?;
    Ok(unindent::unindent(&string))
}
