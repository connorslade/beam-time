use engine::{
    exports::nalgebra::Vector2,
    graphics_context::{Drawable, GraphicsContext},
    render::{shape::ShapeVertex, sprite::GpuSprite},
};

pub struct ColumnLayout {
    origin: Vector2<f32>,
    padding: f32,
}

impl ColumnLayout {
    pub fn new(padding: f32) -> Self {
        Self {
            origin: Vector2::zeros(),
            padding,
        }
    }

    pub fn space_to(&mut self, y: f32) {
        self.origin.y = -y;
    }

    pub fn space(&mut self, height: f32) {
        self.origin.y -= height;
    }

    fn layout(&mut self, (sprites, shapes): (&mut [GpuSprite], &mut [ShapeVertex])) {
        let mut top_left = Vector2::new(f32::INFINITY, f32::NEG_INFINITY);
        for sprite in sprites.iter() {
            for point in sprite.points {
                top_left.x = top_left.x.min(point.x);
                top_left.y = top_left.y.max(point.y);
            }
        }

        for vert in shapes.iter() {
            top_left.x = top_left.x.min(vert.position.x);
            top_left.y = top_left.y.max(vert.position.y);
        }

        let (mut min, mut max) = (f32::INFINITY, f32::NEG_INFINITY);
        let shift = self.origin - top_left;
        for sprite in sprites {
            for x in sprite.points.iter_mut() {
                *x += shift;
                min = min.min(x.y);
                max = max.max(x.y);
            }
        }

        for vert in shapes {
            vert.position += shift;
            min = min.min(vert.position.y);
            max = max.max(vert.position.y);
        }

        self.origin.y -= self.padding + (max - min);
    }

    pub fn draw<App>(&mut self, ctx: &mut GraphicsContext<App>, drawable: impl Drawable<App>) {
        self.layout(ctx.draw_callback(|ctx| ctx.draw(drawable)));
    }

    pub fn row<App>(
        &mut self,
        ctx: &mut GraphicsContext<App>,
        drawable: impl FnOnce(&mut GraphicsContext<App>),
    ) {
        self.layout(ctx.draw_callback(drawable));
    }
}
