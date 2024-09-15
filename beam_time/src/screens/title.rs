use std::time::{Duration, Instant};

use engine::{
    drawable::{sprites::Sprite, text::Text},
    exports::nalgebra::Vector2,
    graphics_context::{Anchor, GraphicsContext},
    screens::Screen,
};

use crate::{
    assets::{COPYRIGHT, DEFAULT_FONT, TITLE},
    consts::BACKGROUND_COLOR,
};

pub struct TitleScreen {
    pub last_update: Instant,
    pub frames: usize,
    pub last_frames: usize,
}

impl Screen for TitleScreen {
    fn render(&mut self, ctx: &mut GraphicsContext) {
        ctx.background(BACKGROUND_COLOR);

        let pos = Vector2::new(ctx.size.x / 2, ctx.size.y * 9 / 10);
        ctx.draw(
            Sprite::builder(TITLE)
                .pos(pos, Anchor::TopCenter)
                .scale(Vector2::repeat(5.0)),
        );

        ctx.draw(
            Sprite::builder(COPYRIGHT)
                .pos(Vector2::new(ctx.size.x - 10, 10), Anchor::BottomRight)
                .scale(Vector2::repeat(2.0)),
        );

        ctx.draw(
            Text::builder(DEFAULT_FONT, "I got text rendering working!")
                .pos(ctx.size / 2, Anchor::Center)
                .scale(Vector2::repeat(5.0)),
        );

        ctx.draw(
            Text::builder(DEFAULT_FONT, "(don't ask how long making the font took)")
                .pos(ctx.size / 2 - Vector2::new(0, 60), Anchor::Center)
                .scale(Vector2::repeat(3.0)),
        );

        self.frames += 1;
        if self.last_update.elapsed() >= Duration::from_secs(1) {
            self.last_frames = self.frames;
            self.frames = 0;
            self.last_update = Instant::now();
        }

        ctx.draw(
            Text::builder(DEFAULT_FONT, &format!("FPS: {:.1}", self.last_frames))
                .pos(Vector2::new(10, 10), Anchor::BottomLeft)
                .scale(Vector2::repeat(2.0)),
        );
    }
}
