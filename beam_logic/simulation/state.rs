use std::{
    borrow::Cow,
    hash::{DefaultHasher, Hash, Hasher},
};

use common::{map::Map, misc::in_bounds};
use log::trace;
use nalgebra::Vector2;

use crate::{
    level::{DynamicElementMap, Level},
    tile::Tile,
};

use super::{level_state::LevelState, tile::BeamTile};

pub struct BeamState {
    pub board: Map<BeamTile>,
    pub level: Option<LevelState>,
    pub bounds: (Vector2<i32>, Vector2<i32>),
}

impl BeamState {
    /// Creates a new BeamState from a Board by converting Tiles into their
    /// BeamTile counterparts.
    pub fn new(tiles: &Map<Tile>, level: Option<Cow<'static, Level>>, test: Option<usize>) -> Self {
        let level = test
            .and_then(|o| level.map(|x| LevelState::new(x, DynamicElementMap::from_map(tiles), o)));

        let mut bounds = (Vector2::repeat(i32::MAX), Vector2::repeat(i32::MIN));
        let board = tiles.map(|pos, tile| {
            bounds.0.x = bounds.0.x.min(pos.x);
            bounds.0.y = bounds.0.y.min(pos.y);
            bounds.1.x = bounds.1.x.max(pos.x);
            bounds.1.y = bounds.1.y.max(pos.y);
            tile.into()
        });

        let mut state = Self {
            board,
            level,
            bounds,
        };

        if let Some(level) = &mut state.level {
            trace!("Running with level: {}", level.level.name);
            level.setup_case(&mut state.board);
        }

        state
    }

    pub fn hash(&self) -> u64 {
        let mut tiles = (self.board.iter())
            .filter(|(pos, _)| in_bounds(*pos, self.bounds))
            .collect::<Vec<_>>();
        tiles.sort_by(|(a, _), (b, _)| a.x.cmp(&b.x).then(a.y.cmp(&b.y)));

        let mut hasher = DefaultHasher::new();
        for (pos, tile) in tiles.iter() {
            pos.hash(&mut hasher);
            tile.hash(&mut hasher);
        }

        hasher.finish()
    }
}
