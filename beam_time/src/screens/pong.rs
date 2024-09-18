use engine::{
    drawable::sprites::Sprite,
    exports::{
        nalgebra::Vector2,
        winit::keyboard::{KeyCode, PhysicalKey},
    },
    graphics_context::{Anchor, GraphicsContext},
    screens::Screen,
};

use crate::{
    assets::{BALL, PADDLE},
    consts::BACKGROUND_COLOR,
};

pub struct PongScreen {
    need_init: bool,
    pos: Vector2<f32>,
    vel: Vector2<f32>,
}

impl Screen for PongScreen {
    fn render(&mut self, ctx: &mut GraphicsContext) {
        ctx.input
            .key_down(PhysicalKey::Code(KeyCode::Escape))
            .then(|| ctx.pop_screen());

        if self.need_init {
            self.need_init = false;
            self.pos = ctx.center();
        }

        let size = ctx.size();
        ctx.background(BACKGROUND_COLOR);

        self.pos += self.vel * ctx.delta_time;

        let width = 8.0 * 5.0 * ctx.scale_factor / 2.0;
        let paddle_height = 16.0 * 5.0 / 2.0;
        if self.pos.x < width
            || self.pos.x > size.x - width
            || (self.pos.x > size.x - width - 37.5 * ctx.scale_factor
                && self.pos.y >= ctx.input.mouse.y - paddle_height
                && self.pos.y <= ctx.input.mouse.y + paddle_height)
        {
            self.vel.x *= -1.0;
        }

        if self.pos.y < width || self.pos.y > size.y - width {
            self.vel.y *= -1.0;
        }
        
        ctx.draw(
            Sprite::new(BALL)
                .pos(self.pos, Anchor::Center)
                .scale(Vector2::repeat(5.0)),
        );

        let paddle_pos = Vector2::new(size.x - 30.0 * ctx.scale_factor, ctx.input.mouse.y);
        ctx.draw(
            Sprite::new(PADDLE)
                .pos(paddle_pos, Anchor::CenterRight)
                .scale(Vector2::repeat(5.0)),
        );
    }
}

impl Default for PongScreen {
    fn default() -> Self {
        Self {
            need_init: true,
            pos: Vector2::new(0.0, 0.0),
            vel: Vector2::new(200.0, 200.0),
        }
    }
}
