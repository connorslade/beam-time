use engine::{
    drawable::sprite::Sprite,
    exports::nalgebra::Vector2,
    graphics_context::{Anchor, GraphicsContext},
};

use crate::{assets::INFO_PANEL, consts::layer};

#[derive(Default)]
pub struct LevelPanel {}

const SIZE: (usize, usize) = (4, 2);

impl LevelPanel {
    pub fn render<App>(&mut self, ctx: &mut GraphicsContext<App>) {
        for y in 0..SIZE.1 {
            for x in 0..SIZE.0 {
                let pos = Vector2::new((x * 16 * 4) as f32, ctx.size().y - (y * 16 * 4) as f32);

                let side = (x == SIZE.0 - 1) as i32 - (x == 0) as i32;
                let uv_offset = Vector2::new(side * 16, 16 * (y == SIZE.1 - 1) as i32);

                ctx.draw(
                    Sprite::new(INFO_PANEL)
                        .scale(Vector2::repeat(4.0), Anchor::Center)
                        .position(pos, Anchor::TopLeft)
                        .uv_offset(uv_offset)
                        .z_index(layer::UI_BACKGROUND),
                );
            }
        }
    }
}
