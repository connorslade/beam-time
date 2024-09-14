use engine::{
    graphics_context::{Anchor, GraphicsContext},
    screens::Screen,
    sprites::Sprite,
};
use nalgebra::Vector2;

use crate::{assets::TITLE, consts::BACKGROUND_COLOR};

pub struct TitleScreen {}

impl Screen for TitleScreen {
    fn render(&mut self, ctx: &mut GraphicsContext) {
        ctx.background(BACKGROUND_COLOR);

        let pos = Vector2::new(ctx.size.x / 2, ctx.size.y * 9 / 10);
        ctx.draw(
            Sprite::builder(TITLE)
                .pos(pos, Anchor::TopCenter)
                .scale(Vector2::repeat(5.0)),
        );
    }
}
