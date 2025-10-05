use engine::{
    color::Rgb,
    drawable::{Anchor, Drawable, shape::rectangle::Rectangle, sprite::Sprite, text::Text},
    exports::nalgebra::Vector2,
    graphics_context::GraphicsContext,
    layout::{LayoutElement, bounds::Bounds2D},
};
use itertools::Itertools;
use leaderboard::api::results;

use crate::assets::{DOWN_ARROW, UNDEAD_FONT};

pub struct Histogram {
    position: Vector2<f32>,
    data: results::Histogram,
    real: Option<u32>,
    title: Option<&'static str>,
}

impl Histogram {
    pub fn new(data: results::Histogram) -> Self {
        Self {
            position: Vector2::zeros(),
            data,
            real: None,
            title: None,
        }
    }

    pub fn real(mut self, real: u32) -> Self {
        self.real = Some(real);
        self
    }

    pub fn title(mut self, title: &'static str) -> Self {
        self.title = Some(title);
        self
    }
}

impl Drawable for Histogram {
    // should prob rewrite this with the layout system at some point...
    fn draw(self, ctx: &mut GraphicsContext) {
        let px = 4.0;
        let bar = Vector2::new(16.0, 3.0);
        let (width, height) = (bar.x * 12.0, 15.0 * px);
        let position = self.position + Vector2::y() * 18.0;

        let max_value = *self.data.bins.iter().max().unwrap() as f32;

        let bars = self.data.bins.into_iter().enumerate();
        let bars = bars
            .map(|(i, value)| {
                let height = value as f32 / max_value * height;
                Vector2::new(bar.x * i as f32, height)
            })
            .collect::<Vec<_>>();

        for (idx, offset) in bars.iter().enumerate() {
            Rectangle::new(bar)
                .position(position + offset + Vector2::y() * 3.0, Anchor::BottomLeft)
                .draw(ctx);
            Rectangle::new(bar + Vector2::x() * 3.0 * (idx + 1 != bars.len()) as u8 as f32)
                .position(position + offset, Anchor::BottomLeft)
                .color(Rgb::hex(0x8d8d8d))
                .draw(ctx);
        }

        for (a, b) in bars.iter().tuple_windows() {
            let height = b.y - a.y;
            let offset = a + Vector2::new(bar.x, 3.0 * (height < 0.0) as u8 as f32);
            Rectangle::new(Vector2::new(3.0, height))
                .position(position + offset + Vector2::y() * 3.0, Anchor::BottomLeft)
                .draw(ctx);
        }

        if let Some(real) = self.real {
            let t = real as f32 / self.data.max as f32;
            let offset = Vector2::new(t * width, height + px * 2.0);
            Sprite::new(DOWN_ARROW)
                .position(position + offset, Anchor::BottomCenter)
                .scale(Vector2::repeat(2.0))
                .draw(ctx);

            let text_offset = offset + Vector2::y() * 12.0;
            Text::new(UNDEAD_FONT, real.to_string())
                .position(position + text_offset, Anchor::BottomCenter)
                .scale(Vector2::repeat(2.0))
                .draw(ctx);
        }

        if let Some(title) = self.title {
            let offset = Vector2::new(
                width / 2.0,
                height + px * 3.0 + 28.0 * self.real.is_some() as u8 as f32,
            );

            Text::new(UNDEAD_FONT, title)
                .position(position + offset, Anchor::BottomCenter)
                .scale(Vector2::repeat(2.0))
                .draw(ctx);
        }

        Text::new(UNDEAD_FONT, "0")
            .position(position - Vector2::y() * px, Anchor::TopLeft)
            .scale(Vector2::repeat(2.0))
            .draw(ctx);
        Text::new(UNDEAD_FONT, self.data.max.to_string())
            .position(position + Vector2::new(width, -px), Anchor::TopRight)
            .scale(Vector2::repeat(2.0))
            .draw(ctx);
    }
}

impl LayoutElement for Histogram {
    fn translate(&mut self, distance: Vector2<f32>) {
        self.position += distance;
    }

    fn bounds(&self, _ctx: &mut GraphicsContext) -> Bounds2D {
        let size = Vector2::new(192.0, 130.0);
        Bounds2D::new(self.position, self.position + size)
    }

    fn draw(self: Box<Self>, ctx: &mut GraphicsContext) {
        (*self).draw(ctx);
    }
}
