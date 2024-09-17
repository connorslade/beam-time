use std::time::{Duration, Instant};

use engine::{
    drawable::{sprites::Sprite, text::Text},
    exports::nalgebra::Vector2,
    graphics_context::{Anchor, GraphicsContext},
    screens::Screen,
};

use crate::{
    assets::{ABOUT_BUTTON, COPYRIGHT, DEFAULT_FONT, OPTIONS_BUTTON, START_BUTTON, TITLE},
    consts::{BACKGROUND_COLOR, FOREGROUND_COLOR},
};

pub struct TitleScreen {
    pub start_time: Instant,
    pub last_update: Instant,
    pub frames: usize,
    pub last_frames: usize,
}

impl Screen for TitleScreen {
    fn render(&mut self, ctx: &mut GraphicsContext) {
        ctx.background(BACKGROUND_COLOR);

        let pos = Vector2::new(ctx.size.x / 2.0, ctx.size.y * 0.9);
        let t = self.start_time.elapsed().as_secs_f32().sin() / 20.0;
        ctx.draw(
            Sprite::new(TITLE)
                .pos(pos, Anchor::TopCenter)
                .scale(Vector2::repeat(6.0))
                .rotate(t),
        );

        ctx.draw(
            Sprite::new(COPYRIGHT)
                .pos(Vector2::new(ctx.size.x - 10.0, 10.0), Anchor::BottomRight)
                .scale(Vector2::repeat(2.0)),
        );

        // ctx.draw(
        //     Button::new(&BUTTON_STYLE, "Start")
        //         .pos(ctx.center())
        //         .scale(Vector2::repeat(4.0)),
        // );

        let start = Sprite::new(START_BUTTON)
            .pos(ctx.center(), Anchor::Center)
            .scale(Vector2::repeat(4.0));
        let start_hover = start.is_hovered(ctx);
        ctx.draw(start.scale(Vector2::repeat(4.0 + if start_hover { 0.2 } else { 0.0 })));

        let options = Sprite::new(OPTIONS_BUTTON)
            .pos(ctx.center() - Vector2::new(0.0, 14.0 * 5.0), Anchor::Center)
            .scale(Vector2::repeat(4.0));
        let options_hover = options.is_hovered(ctx);
        ctx.draw(options.scale(Vector2::repeat(4.0 + if options_hover { 0.2 } else { 0.0 })));

        let about = Sprite::new(ABOUT_BUTTON)
            .pos(
                ctx.center() - Vector2::new(0.0, 2.0 * 14.0 * 5.0),
                Anchor::Center,
            )
            .scale(Vector2::repeat(4.0));
        let about_hover = about.is_hovered(ctx);
        ctx.draw(about.scale(Vector2::repeat(4.0 + if about_hover { 0.2 } else { 0.0 })));

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
    }
}
