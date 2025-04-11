use parking_lot::MutexGuard;

use crate::{
    app::App,
    assets::{HORIZONTAL_RULE, UNDEAD_FONT},
    consts::layer,
    game::board::Board,
};
use beam_logic::simulation::{
    level_state::LevelResult, runtime::asynchronous::InnerAsyncSimulationState,
};
use common::misc::{exp_decay, in_bounds};
use engine::{
    color::Rgb,
    drawable::{sprite::Sprite, text::Text},
    exports::nalgebra::Vector2,
    graphics_context::{Anchor, GraphicsContext},
};

mod level_status;
mod test_case;
use level_status::{level_complete, level_failed, level_info};
use test_case::test_case;

use super::components::modal::{Modal, ModalSides};

pub struct LevelPanel {
    pub case: usize,

    offset: f32,
    previous_result: Option<(LevelResult, u32)>,
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
        ctx: &mut GraphicsContext,
        state: &App,
        board: &Board,
        sim: &MutexGuard<InnerAsyncSimulationState>,
        level_result: &Option<LevelResult>,
    ) {
        let Some(level) = board.transient.level else {
            return;
        };

        let scale = 4.0;
        let tile_size = scale * ctx.scale_factor * 16.0;
        let margin = tile_size / 4.0;
        let padding = 10.0 * ctx.scale_factor;

        let mut ui = UIContext {
            scale,
            margin,
            padding,
            tile_size,
            y: ctx.size().y - margin,
        };

        // todo: Don't re-calc price every frame
        let price = board
            .tiles
            .iter()
            .filter(|(pos, _)| !level.permanent.contains(pos))
            .map(|(_, tile)| tile.price())
            .sum::<u32>();

        level_info(ctx, level, price, &mut ui);
        test_case(self, ctx, level, sim, &mut ui);

        let height = ui.y;

        let dt = ctx.delta_time;
        let (gpu, _shapes) = ctx.draw_callback(|ctx| {
            if let Some((result, price)) = level_result.map(|x| (x, price)).or(self.previous_result)
            {
                self.previous_result = Some((result, price));

                ui.horizontal_rule(ctx);
                match result {
                    LevelResult::Success { latency } => {
                        level_complete(ctx, state, level, latency, price, &mut ui)
                    }
                    LevelResult::Failed { case } => level_failed(ctx, case + 1, &mut ui),
                    LevelResult::OutOfTime => unreachable!(),
                }

                if level_result.is_none() {
                    ui.y = height;
                }
            }
        });

        self.offset = self.offset.min(height);
        self.offset = exp_decay(self.offset, ui.y, 10.0, dt);

        let clip = [
            Vector2::new(0.0, self.offset),
            Vector2::new(f32::MAX, f32::MAX),
        ];
        gpu.iter_mut().for_each(|x| x.clip = clip);

        ui.y = self.offset;
        background(ctx, &mut ui);

        let bounds = (
            Vector2::new(0.0, ui.y),
            Vector2::new(WIDTH as f32 * tile_size, ctx.size().y),
        );
        if in_bounds(ctx.input.mouse, bounds) {
            ctx.input.cancel_clicks();
        }
    }
}

impl UIContext {
    fn text_block(&mut self, ctx: &mut GraphicsContext, text: &str) {
        let text = Text::new(UNDEAD_FONT, text)
            .position(Vector2::new(self.margin, self.y), Anchor::TopLeft)
            .scale(Vector2::repeat(2.0))
            .max_width(WIDTH as f32 * self.tile_size - self.margin * 2.0)
            .z_index(layer::UI_ELEMENT);
        self.y -= text.size(ctx).y + self.padding;
        ctx.draw(text);
    }

    fn horizontal_rule(&mut self, ctx: &mut GraphicsContext) {
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

fn background(ctx: &mut GraphicsContext, ui: &mut UIContext) {
    ui.y -= ui.margin;

    let width = ui.tile_size * WIDTH as f32;
    let height = ctx.size().y - ui.y;

    Modal::new(Vector2::new(width, height))
        .position(Vector2::y() * ctx.size().y, Anchor::TopLeft)
        .layer(layer::UI_BACKGROUND)
        .sides(ModalSides::BOTTOM | ModalSides::RIGHT)
        .popup(false)
        .draw_empty(ctx);
}

impl Default for LevelPanel {
    fn default() -> Self {
        Self {
            case: 0,

            offset: f32::MAX,
            previous_result: None,
        }
    }
}
