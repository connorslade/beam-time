use serde::{Deserialize, Serialize};

use beam_logic::tile::Tile;
use common::{map::Map, user::UserId};

#[derive(Serialize, Deserialize)]
pub struct GetResultsResponse {
    pub cost: Histogram,
    pub latency: Histogram,
}

#[derive(Serialize, Deserialize)]
pub struct PutResults {
    pub user: UserId,
    pub board: Map<Tile>,
}

#[derive(Serialize)]
pub struct PutResultsRef<'a> {
    pub user: &'a UserId,
    pub board: &'a Map<Tile>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Histogram {
    pub bins: [u32; 12],
    pub max: u32,
}

impl Histogram {
    const BIN_COUNT: usize = 12;

    pub fn new(data: &[u32]) -> Self {
        let max = data.iter().copied().max().unwrap_or_default();
        let bin_width = max as f32 / Self::BIN_COUNT as f32;

        let mut bins = [0; Self::BIN_COUNT];
        for &point in data {
            let bin = (point as f32 / bin_width) as usize;
            bins[bin.min(Self::BIN_COUNT - 1)] += 1;
        }

        Self { bins, max }
    }
}
