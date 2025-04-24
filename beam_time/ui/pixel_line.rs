use engine::{
    color::Rgb,
    drawable::shape::rectangle::Rectangle,
    exports::nalgebra::Vector2,
    graphics_context::{Anchor, Drawable, GraphicsContext},
};

pub struct PixelLine {
    a: Vector2<f32>,
    b: Vector2<f32>,

    color: Rgb<f32>,
    z_index: i16,
}

impl PixelLine {
    pub fn new(a: Vector2<f32>, b: Vector2<f32>) -> Self {
        Self {
            a,
            b,
            color: Rgb::repeat(1.0),
            z_index: 0,
        }
    }

    pub fn color(self, color: impl Into<Rgb<f32>>) -> Self {
        Self {
            color: color.into(),
            ..self
        }
    }

    pub fn z_index(self, z_index: i16) -> Self {
        Self { z_index, ..self }
    }
}

impl Drawable for PixelLine {
    // Bresenham's line algorithm, from Wikipedia:
    // https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm
    fn draw(self, ctx: &mut GraphicsContext) {
        let px = 4.0 * ctx.scale_factor;

        let quantize = |val: f32| (val / px).round() as i32;
        let (a, b) = (self.a.map(quantize), self.b.map(quantize));

        let dx = (b.x - a.x).abs();
        let sx = if a.x < b.x { 1 } else { -1 };
        let dy = -(b.y - a.y).abs();
        let sy = if a.y < b.y { 1 } else { -1 };
        let mut error = dx + dy;

        let (mut x0, mut y0) = (a.x, a.y);
        let (x1, y1) = (b.x, b.y);

        while x0 != x1 || y0 != y1 {
            Rectangle::new(Vector2::repeat(px))
                .position(Vector2::new(x0, y0).map(|x| x as f32 * px), Anchor::Center)
                .color(self.color)
                .z_index(self.z_index)
                .draw(ctx);

            let e = 2 * error;
            if e >= dy {
                error += dy;
                x0 += sx;
            }
            if e <= dx {
                error += dx;
                y0 += sy;
            }
        }
    }
}
