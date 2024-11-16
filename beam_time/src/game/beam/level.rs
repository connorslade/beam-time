use crate::{game::level::Level, misc::map::Map};

use super::tile::BeamTile;

pub struct LevelState {
    pub level: &'static Level,
    pub test_case: usize,
    pub cooldown: u32,
}

impl LevelState {
    pub fn is_complete(&self) -> bool {
        self.test_case >= self.level.tests.cases.len()
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

    pub fn case_correct(&mut self, board: &mut Map<BeamTile>) -> bool {
        if self.cooldown > 0 {
            self.cooldown -= 1;
            return false;
        }

        let tests = &self.level.tests;
        let case = &tests.cases[self.test_case];

        for (pos, state) in tests.detectors.iter().zip(&case.detectors) {
            let pos = pos.into_pos();
            let BeamTile::Detector { powered } = board.get(pos) else {
                return false;
            };

            if powered.any() != *state {
                return false;
            }
        }

        true
    }
}
