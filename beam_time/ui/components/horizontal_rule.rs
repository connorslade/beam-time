use engine::{
    color::Rgb,
    drawable::shape::rectangle::Rectangle,
    drawable::{Anchor, Drawable},
    exports::nalgebra::Vector2,
    graphics_context::GraphicsContext,
    layout::{LayoutElement, bounds::Bounds2D},
};

pub struct Rule {
    position: Vector2<f32>,
    direction: RuleDirection,
    width: f32,
    margin: f32,
    z_index: i16,
}

enum RuleDirection {
    Horizontal,
    Vertical,
}

impl Rule {
    pub fn horizontal(width: f32) -> Self {
        Self {
            width,
            direction: RuleDirection::Horizontal,
            position: Vector2::zeros(),
            margin: 0.0,
            z_index: 0,
        }
    }

    pub fn vertical(height: f32) -> Self {
        Self {
            width: height,
            direction: RuleDirection::Vertical,
            position: Vector2::zeros(),
            margin: 0.0,
            z_index: 0,
        }
    }

    pub fn position(self, position: Vector2<f32>) -> Self {
        Self { position, ..self }
    }

    pub fn z_index(self, z_index: i16) -> Self {
        Self { z_index, ..self }
    }

    pub fn margin(self, margin: f32) -> Self {
        Self { margin, ..self }
    }
}

impl Drawable for Rule {
    fn draw(self, ctx: &mut GraphicsContext) {
        let px = 4.0 * ctx.scale_factor;
        let dots = (self.width / px / 2.0) as u32;

        for i in 0..dots {
            let offset = match self.direction {
                RuleDirection::Horizontal => Vector2::new(px * 2.0 * i as f32, self.margin),
                RuleDirection::Vertical => Vector2::new(self.margin, px * 2.0 * i as f32),
            };
            Rectangle::new(Vector2::repeat(px))
                .position(self.position + offset, Anchor::BottomLeft)
                .color(Rgb::repeat(0.459))
                .z_index(self.z_index)
                .draw(ctx);
        }
    }
}

impl LayoutElement for Rule {
    fn translate(&mut self, distance: Vector2<f32>) {
        self.position += distance;
    }

    fn bounds(&self, ctx: &mut GraphicsContext) -> Bounds2D {
        let parallel = 4.0 * ctx.scale_factor + self.margin * 2.0;
        let size = match self.direction {
            RuleDirection::Horizontal => Vector2::new(self.width, parallel),
            RuleDirection::Vertical => Vector2::new(parallel, self.width),
        };
        Bounds2D::new(self.position, self.position + size)
    }

    fn draw(self: Box<Self>, ctx: &mut GraphicsContext) {
        (*self).draw(ctx);
    }
}
