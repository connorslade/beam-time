use std::collections::HashMap;

use log::trace;

use crate::{
    game::level::{Level, LEVELS},
    misc::map::Map,
};

use super::tile::BeamTile;

pub struct LevelState {
    pub level: &'static Level,
    pub test_case: usize,
    pub cooldown: u32,

    pub history: HashMap<u64, usize>,
    // todo: replace with unpacked bitvec
    pub history_states: Vec<Vec<bool>>,
}

impl LevelState {
    pub fn is_complete(&self) -> bool {
        self.test_case >= self.level.tests.cases.len()
    }

    pub fn tick(&mut self, hash: u64, board: &mut Map<BeamTile>) -> bool {
        let idx = self.history_states.len();
        self.history_states.push(self.outputs(board));

        if let Some(idx) = self.history.insert(hash, idx) {
            let cycle = &self.history_states[idx..self.history_states.len() - 1];
            println!("Found Cycle: {:?}", cycle);

            if equivalent_cycles(cycle, &self.level.tests.cases[self.test_case].detectors) {
                trace!("Passed case #{}", self.test_case);
                self.test_case += 1;

                if self.test_case >= self.level.tests.cases.len() {
                    trace!("Passed all cases!");
                    return true;
                }

                self.setup_case(board);
            }
        }

        false
    }

    pub fn setup_case(&mut self, board: &mut Map<BeamTile>) {
        let tests = &self.level.tests;
        let case = &tests.cases[self.test_case];

        if let Some(cooldown) = tests.delay {
            self.cooldown = cooldown;
        }

        for (pos, state) in tests.lasers.iter().zip(&case.lasers) {
            let pos = pos.into_pos();
            if let BeamTile::Emitter { active, .. } = board.get_mut(pos) {
                *active = *state;
            }
        }
    }

    fn outputs(&self, board: &Map<BeamTile>) -> Vec<bool> {
        let tests = &self.level.tests;
        tests
            .detectors
            .iter()
            .map(|pos| {
                let pos = pos.into_pos();
                let BeamTile::Detector { powered } = board.get(pos) else {
                    return false;
                };

                powered.any()
            })
            .collect()
    }
}

fn equivalent_cycles(long: &[Vec<bool>], short: &[Vec<bool>]) -> bool {
    assert!(long.len() >= short.len());

    if long.len() % short.len() != 0 {
        return false;
    }

    for i in 0..(long.len() / short.len()) {
        if &long[i * short.len()..(i + 1) * short.len()] != short {
            return false;
        }
    }

    true
}

impl Default for LevelState {
    fn default() -> Self {
        Self {
            level: &LEVELS[0],
            test_case: Default::default(),
            cooldown: Default::default(),
            history: Default::default(),
            history_states: Default::default(),
        }
    }
}
