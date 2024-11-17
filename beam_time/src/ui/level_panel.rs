use engine::{
    drawable::sprite::Sprite,
    exports::nalgebra::Vector2,
    graphics_context::{Anchor, GraphicsContext},
};

use crate::{app::App, assets::INFO_PANEL, consts::layer};

#[derive(Default)]
pub struct LevelPanel {}

const SIZE: (usize, usize) = (4, 2);

impl LevelPanel {
    pub fn render(&mut self, ctx: &mut GraphicsContext<App>, state: &App) {
        let scale = state.config.ui_scale * 4.0;
        let tile_size = scale * ctx.scale_factor * 16.0;

        for y in 0..SIZE.1 {
            for x in 0..SIZE.0 {
                let pos = Vector2::new(x as f32 * tile_size, ctx.size().y - tile_size * y as f32);

                let side = (x == SIZE.0 - 1) as i32 - (x == 0) as i32;
                let uv_offset = Vector2::new(side * 16, 16 * (y == SIZE.1 - 1) as i32);

                ctx.draw(
                    Sprite::new(INFO_PANEL)
                        .scale(Vector2::repeat(scale), Anchor::Center)
                        .position(pos, Anchor::TopLeft)
                        .uv_offset(uv_offset)
                        .z_index(layer::UI_BACKGROUND),
                );
            }
        }
    }
}
