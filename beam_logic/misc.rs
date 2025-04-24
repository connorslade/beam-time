use common::map::Map;

use crate::{level::Level, tile::Tile};

pub fn price(board: &Map<Tile>, level: Option<&Level>) -> u32 {
    board
        .tiles
        .iter()
        .filter(|(pos, _)| level.map(|x| !x.permanent.contains(pos)).unwrap_or(true))
        .map(|(_, tile)| tile.price())
        .sum::<u32>()
}
