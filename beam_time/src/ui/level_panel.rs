use engine::{
    drawable::{sprite::Sprite, text::Text},
    exports::nalgebra::Vector2,
    graphics_context::{Anchor, GraphicsContext},
};

use crate::{
    app::App,
    assets::{INFO_PANEL, UNDEAD_FONT},
    consts::layer,
    game::board::Board,
};

#[derive(Default)]
pub struct LevelPanel {}

const WIDTH: usize = 6;

impl LevelPanel {
    pub fn render(&mut self, ctx: &mut GraphicsContext<App>, state: &App, board: &Board) {
        let Some(level) = board.transient.level else {
            return;
        };

        let scale = state.config.ui_scale * 4.0;
        let tile_size = scale * ctx.scale_factor * 16.0;

        // Render text
        let padding = tile_size / 4.0;
        let mut y = ctx.size().y - padding;

        let title = Text::new(UNDEAD_FONT, &level.name)
            .position(Vector2::new(padding, y), Anchor::TopLeft)
            .scale(Vector2::repeat(state.config.ui_scale * 3.0))
            .z_index(layer::UI_ELEMENT);
        y -= title.size(ctx).y + 10.0 * state.config.ui_scale * ctx.scale_factor;
        ctx.draw(title);

        let description = Text::new(UNDEAD_FONT, &level.description)
            .position(Vector2::new(padding, y), Anchor::TopLeft)
            .scale(Vector2::repeat(state.config.ui_scale * 2.0))
            .max_width(WIDTH as f32 * tile_size - padding * 2.0)
            .z_index(layer::UI_ELEMENT);
        y -= description.size(ctx).y;
        ctx.draw(description);

        // Render backgrounds
        let height = ((ctx.size().y - y + padding) / tile_size).ceil() as usize;
        for yi in 0..height {
            for xi in 0..WIDTH {
                let mut pos =
                    Vector2::new(xi as f32 * tile_size, ctx.size().y - tile_size * yi as f32);

                if yi + 1 == height {
                    pos.y = y + tile_size - padding;
                }

                let side = (xi == WIDTH - 1) as i32 - (xi == 0) as i32;
                let uv_offset = Vector2::new(side * 16, 16 * (yi == height - 1) as i32);

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
