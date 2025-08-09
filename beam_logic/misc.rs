use common::map::Map;

use crate::{level::Level, tile::Tile};

pub fn price(board: &Map<Tile>, level: &Level) -> (u32, usize) {
    let (mut price, mut count) = (0, 0);
    for (pos, tile) in board.iter() {
        let is_dynamic = tile.id().map(|id| level.is_dynamic(id)).unwrap_or_default();
        if level.permanent.contains(&pos) || is_dynamic {
            continue;
        }

        price += tile.price();
        count += 1;
    }

    (price, count)
}
