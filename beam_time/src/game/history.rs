use std::collections::VecDeque;

use engine::exports::nalgebra::Vector2;

use crate::{consts::MAX_HISTORY, misc::map::Map};

use super::tile::Tile;

pub struct History {
    actions: VecDeque<Action>,
}

pub enum Action {
    Delete { tiles: Vec<(Vector2<i32>, Tile)> },
    Replace { pos: Vector2<i32>, old: Tile },
}

impl History {
    pub fn new() -> Self {
        Self {
            actions: VecDeque::new(),
        }
    }

    pub fn track(&mut self, action: Action) {
        self.actions.push_back(action);

        while self.actions.len() > MAX_HISTORY {
            self.actions.pop_front();
        }
    }

    pub fn pop(&mut self, map: &mut Map<Tile>) {
        if let Some(action) = self.actions.pop_back() {
            action.undo(map);
        }
    }
}

impl Action {
    fn undo(&self, map: &mut Map<Tile>) {
        match self {
            Action::Delete { tiles } => {
                tiles.iter().for_each(|(pos, tile)| map.set(*pos, *tile));
            }
            Action::Replace { pos, old } => map.set(*pos, *old),
        }
    }
}
