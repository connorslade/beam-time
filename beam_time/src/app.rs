use std::time::Instant;

use crate::ui::waterfall::WaterfallState;

pub struct App {
    pub start: Instant,
    pub waterfall: WaterfallState,
}

impl App {
    pub fn frame(&self) -> u8 {
        self.start.elapsed().as_millis() as u8 / 100 % 3
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            start: Instant::now(),
            waterfall: WaterfallState::default(),
        }
    }
}
