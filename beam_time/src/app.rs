use crate::ui::waterfall::WaterfallState;

pub struct App {
    pub waterfall: WaterfallState,
}

impl Default for App {
    fn default() -> Self {
        Self {
            waterfall: WaterfallState::default(),
        }
    }
}
