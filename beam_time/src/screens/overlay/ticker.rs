use engine::{graphics_context::GraphicsContext, screens::Screen};

use crate::app::App;

pub struct Ticker;

impl Screen<App> for Ticker {
    fn pre_render(&mut self, state: &mut App, _ctx: &mut GraphicsContext<App>) {
        state.on_tick();
    }
}
