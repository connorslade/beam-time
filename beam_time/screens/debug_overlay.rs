use std::time::{Duration, Instant};

use engine::{
    drawable::text::Text,
    drawable::{Anchor, Drawable},
    exports::nalgebra::Vector2,
    graphics_context::GraphicsContext,
};

use crate::{App, assets::UNDEAD_FONT, consts::layer::OVERLAY, screens::Screen};

pub struct DebugOverlay {
    last_update: Instant,
    frames: usize,
    last_frames: usize,
}

impl Screen for DebugOverlay {
    fn post_render(&mut self, state: &mut App, ctx: &mut GraphicsContext) {
        self.frames += 1;
        if self.last_update.elapsed() >= Duration::from_secs(1) {
            self.last_frames = self.frames;
            self.frames = 0;
            self.last_update = Instant::now();
        }

        let (fps, sprites, scale) = (
            self.last_frames,
            ctx.sprite_count(),
            ctx.window.scale_factor(),
        );
        let text = if state.config.debug {
            format!(
                "FPS: {fps}\nSprites: {sprites}\nScale: {scale:.1}\n{}",
                state.debug.join("\n")
            )
        } else if state.config.show_fps {
            format!("FPS: {fps}")
        } else {
            String::new()
        };
        state.debug.clear();

        Text::new(UNDEAD_FONT, &text)
            .position(ctx.size() - Vector2::new(10.0, 10.0), Anchor::TopRight)
            .scale(Vector2::repeat(2.0))
            .z_index(OVERLAY)
            .draw(ctx);
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
