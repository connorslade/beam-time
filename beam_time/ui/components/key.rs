use engine::{
    drawable::{sprite::Sprite, text::Text},
    exports::nalgebra::Vector2,
    graphics_context::{Anchor, Drawable, GraphicsContext},
    layout::{LayoutElement, bounds::Bounds2D},
};

use crate::assets::{KEYBOARD_BUTTON, UNDEAD_FONT};

pub struct Key {
    text: &'static str,
    pos: Vector2<f32>,
    scale: f32,
}

impl Key {
    pub fn new(text: &'static str) -> Self {
        Self {
            text,
            pos: Vector2::zeros(),
            scale: 1.0,
        }
    }

    pub fn scale(self, scale: f32) -> Self {
        Self { scale, ..self }
    }
}

impl Drawable for Key {
    fn draw(self, ctx: &mut GraphicsContext) {
        Sprite::new(KEYBOARD_BUTTON)
            .position(self.pos, Anchor::BottomLeft)
            .scale(Vector2::repeat(self.scale))
            .draw(ctx);

        let center = self.pos + Vector2::new(4.0, 6.0) * self.scale * ctx.scale_factor;
        Text::new(UNDEAD_FONT, self.text)
            .position(center, Anchor::Center)
            .scale(Vector2::repeat(self.scale))
            .draw(ctx);
    }
}

impl LayoutElement for Key {
    fn translate(&mut self, distance: Vector2<f32>) {
        self.pos += distance;
    }

    fn bounds(&self, ctx: &mut GraphicsContext) -> Bounds2D {
        let size = Vector2::new(9.0, 11.0) * self.scale * ctx.scale_factor;
        Bounds2D::new(self.pos, self.pos + size)
    }

    fn draw(self: Box<Self>, ctx: &mut GraphicsContext) {
        (*self).draw(ctx);
    }
}
