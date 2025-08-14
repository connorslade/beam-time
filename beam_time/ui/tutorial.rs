use engine::{
    drawable::{Anchor, Drawable, sprite::Sprite, text::Text},
    exports::nalgebra::Vector2,
    graphics_context::GraphicsContext,
    layout::LayoutElement,
    memory::MemoryKey,
    memory_key,
};

use crate::{
    assets::{RIGHT_ARROW, UNDEAD_FONT},
    consts::{layer, spacing::MARGIN},
    ui::components::modal::Modal,
};

pub struct Tutorial {
    text: &'static str,
    point: Vector2<f32>,
    key: MemoryKey,
    width: f32,
}

impl Tutorial {
    pub fn new(text: &'static str, point: Vector2<f32>) -> Self {
        Self {
            key: memory_key!(text),
            width: 300.0,
            text,
            point,
        }
    }

    pub fn key(self, key: MemoryKey) -> Self {
        Self { key, ..self }
    }

    pub fn width(self, width: f32) -> Self {
        Self { width, ..self }
    }
}

impl Drawable for Tutorial {
    fn draw(self, ctx: &mut GraphicsContext) {
        let t = ctx.memory.get_or_insert(self.key, 0.0);
        *t += ctx.delta_time;

        let dx = ((*t * 3.0).sin() + 1.0) / 2.0;
        let offset = -Vector2::x() * ((dx * 2.0).floor() + 1.0) * 4.0;
        Sprite::new(RIGHT_ARROW)
            .scale(Vector2::repeat(4.0))
            .position(self.point + offset, Anchor::CenterRight)
            .z_index(layer::UI_OVERLAY)
            .draw(ctx);

        let text = Text::new(UNDEAD_FONT, self.text)
            .max_width(self.width - MARGIN * 2.0)
            .scale(Vector2::repeat(2.0));
        let size = text.size(ctx) + Vector2::repeat(MARGIN * 2.0);

        let offset = -Vector2::x() * 7.0 * 4.0;
        Modal::new(size)
            .layer(layer::UI_OVERLAY)
            .margin(MARGIN)
            .popup(false)
            .position(self.point + offset, Anchor::CenterRight)
            .draw(ctx, |ctx, layout| {
                text.layout(ctx, layout);
            });
    }
}
