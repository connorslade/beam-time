use std::borrow::Cow;

use ahash::HashMap;
use log::trace;
use serde::{Deserialize, Serialize};

use crate::level::{
    DEFAULT_LEVELS, DynamicElementMap, Level,
    case::{EventType, TestCase},
};
use common::map::Map;

use super::tile::BeamTile;

pub struct LevelState {
    pub(super) level: Cow<'static, Level>,
    dynamic_map: DynamicElementMap,

    pub test_case: usize,
    pub test_offset: usize,

    latency: u32,
    history: HashMap<u64, usize>,
    history_states: Vec<Vec<bool>>,

    pub result: Option<LevelResult>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(tag = "status")]
pub enum LevelResult {
    Success { latency: u32 },
    Failed { case: usize },
    OutOfTime,
}

impl LevelState {
    pub fn new(
        level: Cow<'static, Level>,
        dynamic_map: DynamicElementMap,
        test_offset: usize,
    ) -> Self {
        Self {
            level,
            dynamic_map,
            test_offset,
            ..Default::default()
        }
    }

    pub fn tick(&mut self, hash: u64, board: &mut Map<BeamTile>) {
        let idx = (self.test_case + self.test_offset) % self.level.tests.cases.len();
        let case = &self.level.tests.cases[idx];
        let idx = self.history_states.len();
        self.history_states.push(self.outputs(board));

        match case {
            TestCase::Cycle { detectors, .. } => {
                if let Some(idx) = self.history.insert(hash, idx) {
                    let cycle = &self.history_states[idx..self.history_states.len() - 1];
                    if equivalent_cycles(cycle, detectors) {
                        self.passed_case(self.history_states.len() - cycle.len(), board);
                    } else {
                        self.failed_case();
                    }
                }
            }
            TestCase::Event {
                default,
                pass,
                neutral,
                fail,
                ..
            } => {
                let outputs = self.outputs(board);
                let classification = EventType::classify(pass, neutral, fail, *default, &outputs);

                match classification {
                    EventType::Pass => self.passed_case(self.history_states.len(), board),
                    EventType::Fail => self.failed_case(),
                    EventType::Neutral => {}
                }
            }
        }
    }

    fn passed_case(&mut self, latency: usize, board: &mut Map<BeamTile>) {
        self.latency += latency as u32;
        self.history_states.clear();
        self.history.clear();
        trace!("Passed case #{} {{ latency: {latency} }}", self.test_case);
        self.test_case += 1;

        if self.test_case >= self.level.tests.cases.len() {
            trace!("Passed all cases! {{ latency: {} }}", self.latency);
            self.result = Some(LevelResult::Success {
                latency: self.latency,
            });
            return;
        }

        self.setup_case(board);
    }

    fn failed_case(&mut self) {
        trace!("Failed case #{}", self.test_case);
        self.result = Some(LevelResult::Failed {
            case: self.test_case,
        });
    }

    pub fn setup_case(&mut self, board: &mut Map<BeamTile>) {
        let tests = &self.level.tests;
        let idx = (self.test_case + self.test_offset) % tests.cases.len();
        let case = &tests.cases[idx];

        for (pos, state) in tests.lasers.iter().zip(case.lasers()) {
            let pos = self.dynamic_map.position(*pos);
            if let Some(BeamTile::Emitter { active, .. }) = pos.map(|x| board.get_mut(x)) {
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
                let pos = self.dynamic_map.position(*pos);
                pos.map(|x| board.get(x).is_powered()).unwrap_or_default()
            })
            .collect()
    }
}

fn equivalent_cycles(long: &[Vec<bool>], short: &[Vec<bool>]) -> bool {
    if short.len() > long.len() || !long.len().is_multiple_of(short.len()) {
        return false;
    }

    'outer: for offset in 0..short.len() {
        for i in 0..(long.len() / short.len()) {
            let start = (i * short.len() + offset) % long.len();

            let mut matched = long.iter().cycle().skip(start).zip(short.iter());
            if matched.any(|(a, b)| a != b) {
                continue 'outer;
            }
        }

        return true;
    }

    false
}

impl Default for LevelState {
    fn default() -> Self {
        Self {
            level: Cow::Borrowed(&DEFAULT_LEVELS[0]),
            dynamic_map: Default::default(),
            test_case: 0,
            test_offset: 0,
            latency: 0,
            history: Default::default(),
            history_states: Default::default(),
            result: None,
        }
    }
}
