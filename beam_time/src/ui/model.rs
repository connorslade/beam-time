use engine::{
    drawable::sprite::Sprite,
    exports::nalgebra::Vector2,
    graphics_context::{Anchor, Drawable, GraphicsContext},
};

use crate::assets::PANEL;

type Callback<App> = Box<dyn FnOnce(&mut GraphicsContext<App>)>;

pub struct Modal<App> {
    size: Vector2<f32>,
    layer: i16,

    body: Callback<App>,
}

impl<App> Drawable<App> for Modal<App> {
    fn draw(self, ctx: &mut GraphicsContext<App>) {
        let pos = ctx.center() + Vector2::new(-self.size.x, self.size.y) / 2.0;

        self.background(ctx, pos);

        let sprites = ctx.draw_callback(|ctx| (self.body)(ctx));
        for sprite in sprites {
            sprite.points.iter_mut().for_each(|x| *x += pos);
        }
    }
}

impl<App> Modal<App> {
    pub fn new(
        size: Vector2<f32>,
        layer: i16,
        body: impl FnOnce(&mut GraphicsContext<App>) + 'static,
    ) -> Self {
        Self {
            size,
            layer,
            body: Box::new(body),
        }
    }

    fn background(&self, ctx: &mut GraphicsContext<App>, pos: Vector2<f32>) {
        let scale = 4.0 * ctx.scale_factor;
        let tile_size = 16.0 * scale;

        let y_scale = scale * (self.size.y / tile_size - 2.0);
        let x_scale = scale * (self.size.x / tile_size - 2.0);

        let base = Sprite::new(PANEL)
            .z_index(self.layer)
            .scale(Vector2::repeat(scale));

        ctx.draw([
            // Top
            base.clone()
                .scale(Vector2::repeat(scale))
                .position(pos, Anchor::TopLeft)
                .uv_offset(Vector2::new(-16, -16)),
            base.clone()
                .scale(Vector2::new(x_scale, scale))
                .position(pos + Vector2::x() * tile_size, Anchor::TopLeft)
                .uv_offset(Vector2::new(0, -16)),
            base.clone()
                .scale(Vector2::repeat(scale))
                .position(pos + Vector2::x() * self.size.x, Anchor::TopRight)
                .uv_offset(Vector2::new(16, -16)),
            // Sides
            base.clone()
                .scale(Vector2::new(scale, y_scale))
                .position(pos - Vector2::y() * tile_size, Anchor::TopLeft)
                .uv_offset(Vector2::new(-16, 0)),
            base.clone()
                .scale(Vector2::new(scale, y_scale))
                .position(
                    pos + Vector2::new(self.size.x, -tile_size),
                    Anchor::TopRight,
                )
                .uv_offset(Vector2::new(16, 0)),
            // Bottom
            base.clone()
                .scale(Vector2::repeat(scale))
                .position(pos - Vector2::y() * self.size.y, Anchor::BottomLeft)
                .uv_offset(Vector2::new(-16, 16)),
            base.clone()
                .scale(Vector2::new(x_scale, scale))
                .position(
                    pos + Vector2::new(tile_size, -self.size.y),
                    Anchor::BottomLeft,
                )
                .uv_offset(Vector2::new(0, 16)),
            base.clone()
                .scale(Vector2::repeat(scale))
                .position(
                    pos + Vector2::new(self.size.x, -self.size.y),
                    Anchor::BottomRight,
                )
                .uv_offset(Vector2::new(16, 16)),
            // Middle
            base.scale(Vector2::new(x_scale, y_scale))
                .position(pos + Vector2::new(tile_size, -tile_size), Anchor::TopLeft),
        ]);
    }
}
