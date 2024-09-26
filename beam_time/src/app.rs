use std::time::Instant;

use crate::ui::waterfall::WaterfallState;

pub struct App {
    pub start: Instant,
    pub waterfall: WaterfallState,
}

impl Default for App {
    fn default() -> Self {
        Self {
            start: Instant::now(),
            waterfall: WaterfallState::default(),
        }
    }
}
