use engine::{graphics_context::GraphicsContext, screens::Screen};

use crate::{
    ui::{button::ButtonState, misc::titled_screen},
    App,
};

#[derive(Default)]
pub struct OptionsScreen {
    back_button: ButtonState,
}

impl Screen<App> for OptionsScreen {
    fn render(&mut self, state: &mut App, ctx: &mut GraphicsContext<App>) {
        titled_screen(state, ctx, Some(&mut self.back_button), "Options");
    }
}
