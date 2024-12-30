use std::collections::VecDeque;

use beam_logic::tile::Tile;
use common::map::Map;
use engine::exports::nalgebra::Vector2;

use crate::consts::MAX_HISTORY;

type Action = Vec<(Vector2<i32>, Tile)>;

pub struct History {
    actions: VecDeque<Action>,
    dirty: bool,
}

impl History {
    pub fn new() -> Self {
        Self {
            actions: VecDeque::new(),
            dirty: false,
        }
    }

    pub fn mark_clean(&mut self) {
        self.dirty = false;
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    pub fn track_one(&mut self, pos: Vector2<i32>, old: Tile) {
        self.track_many(vec![(pos, old)]);
    }

    pub fn track_many(&mut self, tiles: Vec<(Vector2<i32>, Tile)>) {
        self.actions.push_back(tiles);
        self.dirty = true;

        while self.actions.len() > MAX_HISTORY {
            self.actions.pop_front();
        }
    }

    pub fn pop(&mut self, map: &mut Map<Tile>) {
        if let Some(action) = self.actions.pop_back() {
            self.dirty = true;
            action.iter().for_each(|(pos, tile)| map.set(*pos, *tile));
        }
    }
}
