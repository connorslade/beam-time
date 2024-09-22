use engine::{graphics_context::GraphicsContext, screens::Screen};

use crate::{
    ui::{button::ButtonState, misc::titled_screen},
    App,
};

#[derive(Default)]
pub struct LevelsScreen {
    back_button: ButtonState,
}

impl Screen<App> for LevelsScreen {
    fn render(&mut self, state: &mut App, ctx: &mut GraphicsContext<App>) {
        titled_screen(state, ctx, &mut self.back_button, "Levels");
    }
}
