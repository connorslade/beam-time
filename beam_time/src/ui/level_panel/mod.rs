use parking_lot::MutexGuard;

use crate::{
    app::App,
    assets::{HORIZONTAL_RULE, INFO_PANEL, UNDEAD_FONT},
    consts::layer,
    game::board::Board,
};
use beam_logic::simulation::{
    level_state::LevelResult, runtime::asynchronous::InnerAsyncSimulationState,
};
use common::misc::in_bounds;
use engine::{
    color::Rgb,
    drawable::{sprite::Sprite, text::Text},
    exports::{nalgebra::Vector2, winit::event::MouseButton},
    graphics_context::{Anchor, GraphicsContext},
};

mod level_status;
mod test_case;
use level_status::{level_complete, level_failed, level_info};
use test_case::test_case;

#[derive(Default)]
pub struct LevelPanel {
    case: usize,
}

struct UIContext {
    scale: f32,
    margin: f32,
    padding: f32,
    tile_size: f32,
    y: f32,
}

const WIDTH: usize = 7;

impl LevelPanel {
    pub fn render(
        &mut self,
        ctx: &mut GraphicsContext<App>,
        state: &App,
        board: &Board,
        sim: &MutexGuard<InnerAsyncSimulationState>,
        level_result: &Option<LevelResult>,
    ) {
        let Some(level) = board.transient.level else {
            return;
        };

        let scale = state.config.ui_scale * 4.0;
        let tile_size = scale * ctx.scale_factor * 16.0;
        let margin = tile_size / 4.0;
        let padding = 10.0 * state.config.ui_scale * ctx.scale_factor;

        let mut ui = UIContext {
            scale,
            margin,
            padding,
            tile_size,
            y: ctx.size().y - margin,
        };

        // todo: Dont re-calc price every frame
        let price = board
            .tiles
            .iter()
            .filter(|(pos, _)| !level.permanent.contains(pos))
            .map(|(_, tile)| tile.price())
            .sum::<u32>();

        level_info(ctx, state, level, price, &mut ui);
        ui.horizontal_rule(ctx);
        test_case(self, ctx, state, level, sim, &mut ui);

        if let Some(result) = level_result {
            ui.horizontal_rule(ctx);
            match result {
                LevelResult::Success { latency } => {
                    level_complete(ctx, state, level, *latency, price, &mut ui)
                }
                LevelResult::Failed { case } => level_failed(ctx, state, case + 1, &mut ui),
                LevelResult::OutOfTime => unreachable!(),
            }
        }

        background(ctx, &mut ui);

        let bounds = (
            Vector2::new(0.0, ui.y),
            Vector2::new(WIDTH as f32 * tile_size, ctx.size().y),
        );
        if in_bounds(ctx.input.mouse, bounds) {
            ctx.input.cancel_mouse(MouseButton::Left);
            ctx.input.cancel_mouse(MouseButton::Right);
        }
    }
}

impl UIContext {
    fn text_block(&mut self, ctx: &mut GraphicsContext<App>, state: &App, text: &str) {
        let text = Text::new(UNDEAD_FONT, text)
            .position(Vector2::new(self.margin, self.y), Anchor::TopLeft)
            .scale(Vector2::repeat(state.config.ui_scale * 2.0))
            .max_width(WIDTH as f32 * self.tile_size - self.margin * 2.0)
            .z_index(layer::UI_ELEMENT);
        self.y -= text.size(ctx).y + self.padding;
        ctx.draw(text);
    }

    fn horizontal_rule(&mut self, ctx: &mut GraphicsContext<App>) {
        self.y -= self.margin;
        for i in 0..WIDTH {
            ctx.draw(
                Sprite::new(HORIZONTAL_RULE)
                    .scale(Vector2::repeat(self.scale))
                    .position(
                        Vector2::new(self.tile_size * i as f32, self.y),
                        Anchor::TopLeft,
                    )
                    .color(Rgb::repeat(0.459))
                    .z_index(layer::UI_ELEMENT),
            );
        }
        self.y -= self.scale + self.margin;
    }
}

fn background(ctx: &mut GraphicsContext<App>, ui: &mut UIContext) {
    ui.y -= ui.margin;
    let height = ctx.size().y - ui.y - ui.tile_size;

    let y_scale = height / (16.0 * ctx.scale_factor);
    let x_scale = ui.scale * (WIDTH - 2) as f32;
    let x_right = ui.tile_size * WIDTH as f32;

    let base = Sprite::new(INFO_PANEL)
        .z_index(layer::UI_BACKGROUND)
        .scale(Vector2::repeat(ui.scale));

    if height > 0.0 {
        ctx.draw([
            base.clone()
                .scale(Vector2::new(ui.scale, y_scale))
                .position(Vector2::new(0.0, ctx.size().y), Anchor::TopLeft)
                .uv_offset(Vector2::new(-16, 0)),
            base.clone()
                .scale(Vector2::new(ui.scale, y_scale))
                .position(Vector2::new(x_right, ctx.size().y), Anchor::TopRight)
                .uv_offset(Vector2::new(16, 0)),
            base.clone()
                .scale(Vector2::new(x_scale, y_scale))
                .position(Vector2::new(ui.tile_size, ctx.size().y), Anchor::TopLeft),
        ]);
    }

    ctx.draw([
        base.clone()
            .position(Vector2::new(0.0, ui.y), Anchor::BottomLeft)
            .uv_offset(Vector2::new(-16, 16)),
        base.clone()
            .position(Vector2::new(x_right, ui.y), Anchor::BottomRight)
            .uv_offset(Vector2::new(16, 16)),
        base.scale(Vector2::new(x_scale, ui.scale))
            .position(Vector2::new(ui.tile_size, ui.y), Anchor::BottomLeft)
            .uv_offset(Vector2::new(0, 16)),
    ]);
}
