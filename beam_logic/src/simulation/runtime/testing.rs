use std::borrow::Cow;

use common::map::Map;

use crate::{level::Level, simulation::state::BeamState, tile::Tile};

pub struct TestingSimulationState {
    beam: BeamState,
    max_ticks: u32,
}

pub struct TestResults {
    tests: Vec<TestResult>,
    latency: Option<u32>,
}

pub enum TestResult {
    Success,
    Failed,
    OutOfTime,
}

impl TestingSimulationState {
    pub fn new(board: &Map<Tile>, level: Cow<'static, Level>, max_ticks: u32) -> Self {
        Self {
            beam: BeamState::new(board, Some(level), true),
            max_ticks,
        }
    }

    pub fn run(&mut self) -> TestResults {
        let mut tests = Vec::new();
        let (mut case, mut timer) = (0, 0);

        loop {
            self.beam.tick();

            let level = self.beam.level.as_ref().unwrap();

            if level.test_case != case {
                tests.push(TestResult::Success);
                timer = 0;
                case = level.test_case;
            }

            timer += 1;

            if timer > self.max_ticks {
                tests.push(TestResult::OutOfTime);
                return TestResults {
                    tests,
                    latency: None,
                };
            }

            let Some(result) = level.complete() else {
                continue;
            };

            return TestResults {
                tests,
                latency: Some(result.latency),
            };
        }
    }
}
