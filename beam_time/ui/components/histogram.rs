use engine::{
    drawable::{shape::rectangle::Rectangle, sprite::Sprite, text::Text},
    exports::nalgebra::Vector2,
    graphics_context::{Anchor, Drawable, GraphicsContext},
    layout::{LayoutElement, bounds::Bounds2D},
};
use itertools::Itertools;
use leaderboard::api::results;

use crate::assets::{HISTOGRAM_MARKER, UNDEAD_FONT};

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
    fn draw(self, ctx: &mut GraphicsContext) {
        let px = 4.0 * ctx.scale_factor;
        let bar = Vector2::new(4.0, 1.0) * px;
        let (width, height) = (bar.x * 12.0, 15.0 * px);
        let position = self.position + Vector2::y() * (px + 12.0 * ctx.scale_factor);

        let max_value = *self.data.bins.iter().max().unwrap() as f32;

        let bars = self.data.bins.into_iter().enumerate();
        let bars = bars
            .map(|(i, value)| {
                let height = value as f32 / max_value * height;
                Vector2::new(bar.x * i as f32, height)
            })
            .collect::<Vec<_>>();

        for offset in bars.iter() {
            Rectangle::new(bar)
                .position(position + offset, Anchor::BottomLeft)
                .draw(ctx);
        }

        for (a, b) in bars.iter().tuple_windows() {
            let height = b.y - a.y;
            let offset = a + Vector2::new(bar.x, px * (height < 0.0) as u8 as f32);
            Rectangle::new(Vector2::new(px, height))
                .position(position + offset, Anchor::BottomLeft)
                .draw(ctx);
        }

        if let Some(real) = self.real {
            let t = real as f32 / self.data.max as f32;
            let offset = Vector2::new(t * width, height + px * 2.0);
            Sprite::new(HISTOGRAM_MARKER)
                .position(position + offset, Anchor::BottomCenter)
                .scale(Vector2::repeat(2.0))
                .draw(ctx);

            let text_offset = offset + Vector2::y() * (6.0 * ctx.scale_factor + px);
            Text::new(UNDEAD_FONT, real.to_string())
                .position(position + text_offset, Anchor::BottomCenter)
                .scale(Vector2::repeat(2.0))
                .draw(ctx);
        }

        if let Some(title) = self.title {
            let offset = Vector2::new(
                width / 2.0,
                height + px * 3.0 + 26.0 * ctx.scale_factor * self.real.is_some() as u8 as f32,
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

    fn bounds(&self, ctx: &mut GraphicsContext) -> Bounds2D {
        let size = (Vector2::new(12.0, 5.0) * 16.0
            + Vector2::y() * 26.0 * self.real.is_some() as u8 as f32
            + Vector2::y() * 20.0 * self.title.is_some() as u8 as f32)
            * ctx.scale_factor;
        Bounds2D::new(self.position, self.position + size)
    }

    fn draw(self: Box<Self>, ctx: &mut GraphicsContext) {
        (*self).draw(ctx);
    }
}
