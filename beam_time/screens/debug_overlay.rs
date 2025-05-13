use std::time::{Duration, Instant};

use engine::{
    drawable::text::Text,
    exports::{nalgebra::Vector2, winit::keyboard::KeyCode},
    graphics_context::{Anchor, Drawable, GraphicsContext},
};

use crate::{assets::UNDEAD_FONT, consts::layer::OVERLAY, screens::Screen, App};

pub struct DebugOverlay {
    last_update: Instant,
    frames: usize,
    last_frames: usize,
}

impl Screen for DebugOverlay {
    fn pre_render(&mut self, _state: &mut App, ctx: &mut GraphicsContext) {
        if ctx.input.key_down(KeyCode::Slash) {
            ctx.scale_factor = 1.0 + (ctx.scale_factor == 1.0) as u8 as f32;
        }
    }

    fn post_render(&mut self, state: &mut App, ctx: &mut GraphicsContext) {
        self.frames += 1;
        if self.last_update.elapsed() >= Duration::from_secs(1) {
            self.last_frames = self.frames;
            self.frames = 0;
            self.last_update = Instant::now();
        }

        let (fps, sprites, scale) = (self.last_frames, ctx.sprite_count(), ctx.scale_factor);
        let text = format!(
            "FPS: {fps}\nSprites: {sprites}\nScale: {scale:.1}\n{}",
            state.debug.join("\n")
        );
        state.debug.clear();

        let pos = ctx.size() - Vector2::new(10.0, 10.0) * scale;
        Text::new(UNDEAD_FONT, &text)
            .position(pos, Anchor::TopRight)
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
