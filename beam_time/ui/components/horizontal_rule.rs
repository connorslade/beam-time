use engine::{
    color::Rgb,
    drawable::shape::rectangle::Rectangle,
    exports::nalgebra::Vector2,
    graphics_context::{Anchor, Drawable, GraphicsContext},
    layout::{bounds::Bounds2D, LayoutElement},
};

pub struct HorizontalRule {
    position: Vector2<f32>,
    width: f32,
    margin: f32,
    z_index: i16,
}

impl HorizontalRule {
    pub fn new(width: f32) -> Self {
        Self {
            width,
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

impl Drawable for HorizontalRule {
    fn draw(self, ctx: &mut GraphicsContext) {
        let px = 4.0 * ctx.scale_factor;
        let dots = (self.width / px / 2.0) as u32;

        for i in 0..dots {
            let offset = Vector2::new(px * 2.0 * i as f32, self.margin);
            Rectangle::new(Vector2::repeat(px))
                .position(self.position + offset, Anchor::BottomLeft)
                .color(Rgb::repeat(0.459))
                .z_index(self.z_index)
                .draw(ctx);
        }
    }
}

impl LayoutElement for HorizontalRule {
    fn translate(&mut self, distance: Vector2<f32>) {
        self.position += distance;
    }

    fn bounds(&self, ctx: &mut GraphicsContext) -> Bounds2D {
        let size = Vector2::new(self.width, 4.0 * ctx.scale_factor + self.margin * 2.0);
        Bounds2D::new(self.position, self.position + size)
    }

    fn draw(self: Box<Self>, ctx: &mut GraphicsContext) {
        (*self).draw(ctx);
    }
}
