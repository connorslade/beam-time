use std::{borrow::Cow, collections::HashMap};

use log::trace;
use serde::{Deserialize, Serialize};

use crate::level::{
    case::{EventType, TestCase},
    Level, DEFAULT_LEVELS,
};
use common::map::Map;

use super::tile::BeamTile;

pub struct LevelState {
    pub(super) level: Cow<'static, Level>,
    pub test_case: usize,

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
    pub fn new(level: Cow<'static, Level>) -> Self {
        Self {
            level,
            ..Default::default()
        }
    }

    pub fn tick(&mut self, hash: u64, board: &mut Map<BeamTile>) {
        if self.result.is_some() {
            return;
        }

        let case = &self.level.tests.cases[self.test_case];
        match case {
            TestCase::Cycle { detectors, .. } => {
                let idx = self.history_states.len();
                self.history_states.push(self.outputs(board));

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
        let case = &tests.cases[self.test_case];

        for (pos, state) in tests.lasers.iter().zip(case.lasers()) {
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
    if short.len() > long.len() || long.len() % short.len() != 0 {
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
            test_case: Default::default(),
            latency: 0,
            history: Default::default(),
            history_states: Default::default(),
            result: None,
        }
    }
}
