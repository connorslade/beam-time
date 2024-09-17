use std::{
    f32::consts::TAU,
    time::{Duration, Instant},
};

use engine::{
    drawable::{sprites::Sprite, text::Text},
    exports::nalgebra::Vector2,
    graphics_context::{Anchor, GraphicsContext},
    screens::Screen,
};

use crate::{
    assets::{BALL, COPYRIGHT, DEFAULT_FONT, PADDLE, TITLE},
    consts::{BACKGROUND_COLOR, FOREGROUND_COLOR, START_COLOR},
};

pub struct TitleScreen {
    pub start_time: Instant,
    pub last_update: Instant,
    pub frames: usize,
    pub last_frames: usize,

    pub pos: Vector2<f32>,
    pub vel: Vector2<f32>,
}

impl Screen for TitleScreen {
    fn render(&mut self, ctx: &mut GraphicsContext) {
        ctx.background(BACKGROUND_COLOR);

        let pos = Vector2::new(ctx.size.x / 2.0, ctx.size.y * 0.9);
        let t = self.start_time.elapsed().as_secs_f32().sin() / 8.0;
        ctx.draw(
            Sprite::new(TITLE)
                .pos(pos, Anchor::TopCenter)
                .scale(Vector2::repeat(5.0))
                .rotate(t),
        );

        ctx.draw(
            Sprite::new(COPYRIGHT)
                .pos(Vector2::new(ctx.size.x - 10.0, 10.0), Anchor::BottomRight)
                .scale(Vector2::repeat(2.0)),
        );

        ctx.draw(
            Text::new(DEFAULT_FONT, "I got text rendering working!")
                .color(FOREGROUND_COLOR)
                .pos(ctx.center(), Anchor::Center)
                .scale(Vector2::repeat(5.0)),
        );

        ctx.draw(
            Text::new(DEFAULT_FONT, "(don't ask how long making the font took)")
                .color(FOREGROUND_COLOR)
                .pos(
                    ctx.size / 2.0 - Vector2::new(0.0, 60.0 * ctx.scale_factor),
                    Anchor::Center,
                )
                .scale(Vector2::repeat(3.0)),
        );

        self.frames += 1;
        if self.last_update.elapsed() >= Duration::from_secs(1) {
            self.last_frames = self.frames;
            self.frames = 0;
            self.last_update = Instant::now();
        }

        ctx.draw(
            Text::new(DEFAULT_FONT, &format!("FPS: {:.1}", self.last_frames))
                .color(FOREGROUND_COLOR)
                .pos(Vector2::new(10.0, 10.0), Anchor::BottomLeft)
                .scale(Vector2::repeat(2.0)),
        );

        self.pos += self.vel * ctx.delta_time;

        let width = 8.0 * 5.0;
        if self.pos.x < width || self.pos.x > ctx.size.x - width {
            self.vel.x *= -1.0;
        }

        if self.pos.y < width || self.pos.y > ctx.size.y - width {
            self.vel.y *= -1.0;
        }

        let t = (self.start_time.elapsed().as_secs_f32() / 8.0).sin() * TAU;
        ctx.draw(
            Sprite::new(BALL)
                .pos(self.pos, Anchor::Center)
                .scale(Vector2::repeat(5.0))
                .color(START_COLOR.hue_shift(t))
                .z_index(1.0),
        );

        let paddle_pos = Vector2::new(ctx.size.x - 30.0 * ctx.scale_factor, ctx.mouse.y);
        ctx.draw(
            Sprite::new(PADDLE)
                .pos(paddle_pos, Anchor::CenterRight)
                .scale(Vector2::repeat(5.0)),
        );
    }
}
