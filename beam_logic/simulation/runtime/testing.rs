use std::borrow::Cow;

use crate::{
    level::Level,
    simulation::{level_state::LevelResult, state::BeamState},
    tile::Tile,
};
use common::map::Map;

pub struct TestingSimulationState {
    beam: BeamState,
    max_ticks: u32,
}

impl TestingSimulationState {
    pub fn new(board: &Map<Tile>, level: Cow<'static, Level>, max_ticks: u32) -> Self {
        Self {
            beam: BeamState::new(board, Some(level), Some(0)),
            max_ticks,
        }
    }

    pub fn run(&mut self) -> LevelResult {
        let (mut case, mut timer) = (0, 0);

        loop {
            self.beam.tick();

            let level = self.beam.level.as_ref().unwrap();

            if level.test_case != case {
                timer = 0;
                case = level.test_case;
            }

            timer += 1;

            if timer > self.max_ticks {
                return LevelResult::OutOfTime;
            }

            if let Some(result) = level.result {
                return result;
            }
        }
    }
}
