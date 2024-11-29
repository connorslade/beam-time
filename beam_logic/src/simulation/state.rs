use std::{
    borrow::Cow,
    hash::{DefaultHasher, Hash, Hasher},
};

use common::{direction::Directions, map::Map, misc::in_bounds};
use log::trace;
use nalgebra::Vector2;

use crate::{level::Level, tile::Tile};

use super::{level_state::LevelState, tile::BeamTile};

pub struct BeamState {
    pub board: Map<BeamTile>,
    pub level: Option<LevelState>,
}

impl BeamState {
    /// Creates a new BeamState from a Board by converting Tiles into their
    /// BeamTile counterparts.
    pub fn new(tiles: &Map<Tile>, level: Option<Cow<'static, Level>>, test: bool) -> Self {
        let level = test
            .then(|| {
                level.map(|level| LevelState {
                    level,
                    ..Default::default()
                })
            })
            .flatten();

        let board = tiles.map(|x| match x {
            Tile::Empty => BeamTile::Empty,
            Tile::Emitter { rotation, active } => BeamTile::Emitter {
                direction: rotation,
                active,
            },
            Tile::Detector => BeamTile::Detector {
                powered: Directions::empty(),
            },
            Tile::Delay => BeamTile::Delay {
                powered: Directions::empty(),
                last_powered: Directions::empty(),
            },
            Tile::Mirror { rotation } => BeamTile::Mirror {
                direction: rotation,
                original_direction: rotation,
                powered: [None; 2],
            },
            Tile::Splitter { rotation } => BeamTile::Splitter {
                direction: rotation,
                powered: None,
            },
            Tile::Galvo { rotation } => BeamTile::Galvo {
                direction: rotation,
                powered: Directions::empty(),
            },
            Tile::Wall { .. } => BeamTile::Wall {
                powered: Directions::empty(),
            },
        });

        let mut state = Self { board, level };

        if let Some(level) = &mut state.level {
            trace!("Running with level: {}", level.level.name);
            level.setup_case(&mut state.board);
        }

        state
    }

    pub fn hash(&self) -> u64 {
        let size = self.level.as_ref().unwrap().level.size.unwrap();
        let bounds = (Vector2::zeros(), size.map(|x| x as i32));

        let mut tiles = self.board.iter().collect::<Vec<_>>();
        tiles.sort_by(|(a, _), (b, _)| a.x.cmp(&b.x).then(a.y.cmp(&b.y)));

        let mut hasher = DefaultHasher::new();
        for (pos, tile) in tiles.iter().filter(|(pos, _)| in_bounds(*pos, bounds)) {
            pos.hash(&mut hasher);
            tile.hash(&mut hasher);
        }

        hasher.finish()
    }
}
