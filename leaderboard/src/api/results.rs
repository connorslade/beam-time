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

#[derive(Clone, Serialize, Deserialize)]
pub struct Histogram {
    pub bins: [u32; 12],
    pub max: u32,
}
