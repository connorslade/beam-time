use std::collections::VecDeque;

use beam_logic::tile::Tile;
use common::map::Map;
use engine::exports::nalgebra::Vector2;

use crate::consts::MAX_HISTORY;

pub struct History {
    actions: VecDeque<Action>,
    dirty: bool,
}

enum Action {
    Many { tiles: Vec<(Vector2<i32>, Tile)> },
    One { pos: Vector2<i32>, old: Tile },
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
        self.track(Action::One { pos, old });
    }

    pub fn track_many(&mut self, tiles: Vec<(Vector2<i32>, Tile)>) {
        self.track(Action::Many { tiles });
    }

    fn track(&mut self, action: Action) {
        self.actions.push_back(action);
        self.dirty = true;

        while self.actions.len() > MAX_HISTORY {
            self.actions.pop_front();
        }
    }

    pub fn pop(&mut self, map: &mut Map<Tile>) {
        if let Some(action) = self.actions.pop_back() {
            self.dirty = true;
            action.undo(map);
        }
    }
}

impl Action {
    fn undo(&self, map: &mut Map<Tile>) {
        match self {
            Action::Many { tiles } => {
                tiles.iter().for_each(|(pos, tile)| map.set(*pos, *tile));
            }
            Action::One { pos, old } => map.set(*pos, *old),
        }
    }
}
