use serde::{Deserialize, Serialize};

use beam_logic::tile::Tile;
use common::map::Map;

#[derive(Serialize, Deserialize)]
pub struct GetResultsResponse {
    pub cost: Histogram,
    pub latency: Histogram,
}

#[derive(Serialize, Deserialize)]
pub struct PutResults {
    pub user: (),
    pub board: Map<Tile>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Histogram {
    pub bins: [u32; 12],
    pub max: u32,
}
