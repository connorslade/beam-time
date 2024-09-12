use crate::{assets::TITLE, graphics_context::GraphicsContext};

use super::Screen;

pub struct TitleScreen {}

impl Screen for TitleScreen {
    fn render(&mut self, ctx: &mut GraphicsContext) {
        ctx.draw_sprite(TITLE, ctx.center());
    }
}
