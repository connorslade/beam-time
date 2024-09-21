use std::time::{Duration, Instant};

use engine::{
    drawable::text::Text,
    exports::nalgebra::Vector2,
    graphics_context::{Anchor, GraphicsContext},
    screens::Screen,
};

use crate::{assets::DEFAULT_FONT, consts::FOREGROUND_COLOR};

pub struct DebugOverlay {
    last_update: Instant,
    frames: usize,
    last_frames: usize,
}

impl Screen for DebugOverlay {
    fn render(&mut self, _ctx: &mut GraphicsContext) {}

    fn update(&mut self, ctx: &mut GraphicsContext) {
        self.frames += 1;
        if self.last_update.elapsed() >= Duration::from_secs(1) {
            self.last_frames = self.frames;
            self.frames = 0;
            self.last_update = Instant::now();
        }

        ctx.draw(
            Text::new(DEFAULT_FONT, &format!("FPS: {:.1}", self.last_frames))
                .color(FOREGROUND_COLOR)
                .pos(Vector2::new(10.0, ctx.size().y - 10.0), Anchor::TopLeft)
                .scale(Vector2::repeat(2.0)),
        );
    }
}

impl Default for DebugOverlay {
    fn default() -> Self {
        Self {
            last_update: Instant::now(),
            frames: 0,
            last_frames: 0,
        }
    }
}
