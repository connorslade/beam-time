use common::misc::exp_decay;
use parking_lot::MutexGuard;
use thousands::Separable;

use crate::{app::App, assets::UNDEAD_FONT, consts::layer, game::board::Board};
use beam_logic::{
    level::Level,
    simulation::{level_state::LevelResult, runtime::asynchronous::InnerAsyncSimulationState},
};
use engine::{
    drawable::{spacer::Spacer, text::Text},
    exports::nalgebra::Vector2,
    graphics_context::{Anchor, GraphicsContext},
    layout::{column::ColumnLayout, tracker::LayoutTracker, Layout, LayoutElement, LayoutMethods},
    memory_key,
};

use super::components::{
    horizontal_rule::HorizontalRule,
    modal::{Modal, ModalSides},
};

mod level_status;
mod test_case;

pub struct LevelPanel {
    pub case: usize,

    height: f32,
    previous_result: Option<(LevelResult, u32)>,
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

        let tile_size = 4.0 * 16.0 * ctx.scale_factor;
        let (margin, padding) = state.spacing(ctx);
        let width = tile_size * WIDTH as f32;

        let price = price(board, level);

        let trackers @ [base, extended] = [memory_key!(), memory_key!()].map(LayoutTracker::new);
        let tracker = trackers[level_result.is_some() as usize];

        let height = tracker
            .bounds(ctx)
            .map(|x| ctx.size().y - x.min.y)
            .unwrap_or_default();
        if self.height == 0.0 {
            self.height = height;
        }
        self.height = exp_decay(self.height, height, 10.0, ctx.delta_time);

        let position = Vector2::new(4.0 * ctx.scale_factor, ctx.size().y);
        let dummy = || Spacer::new_y(-padding);
        Modal::new(Vector2::new(width, self.height + padding))
            .position(position, Anchor::TopLeft)
            .layer(layer::UI_BACKGROUND)
            .sides(ModalSides::LEFT | ModalSides::BOTTOM | ModalSides::RIGHT)
            .margin(margin)
            .popup(false)
            .draw(ctx, |ctx, layout| {
                layout.nest(ctx, ColumnLayout::new(padding), |ctx, layout| {
                    self.level_info(ctx, layout, level, price);
                    self.test_case(ctx, layout, level, sim);
                    dummy().tracked(base).layout(ctx, layout);
                    self.level_status(ctx, state, layout, level, level_result, price);
                    dummy().tracked(extended).layout(ctx, layout);
                });
            });
    }

    fn level_info(
        &mut self,
        ctx: &mut GraphicsContext,
        layout: &mut ColumnLayout,
        level: &Level,
        price: u32,
    ) {
        Text::new(UNDEAD_FONT, &level.name)
            .scale(Vector2::repeat(3.0))
            .layout(ctx, layout);

        let description = format!("${}\n\n{}", price.separate_with_commas(), level.description);
        Text::new(UNDEAD_FONT, description)
            .max_width(layout.available().x)
            .scale(Vector2::repeat(2.0))
            .layout(ctx, layout);
    }
}

// todo: Don't re-calc price every frame
fn price(board: &Board, level: &Level) -> u32 {
    board
        .tiles
        .iter()
        .filter(|(pos, _)| !level.permanent.contains(pos))
        .map(|(_, tile)| tile.price())
        .sum::<u32>()
}

fn horizontal_rule(ctx: &mut GraphicsContext, layout: &mut ColumnLayout) {
    let margin = 12.0 * ctx.scale_factor;
    HorizontalRule::new(layout.available().x + margin * 2.0)
        .position(Vector2::x() * -margin)
        .margin(margin / 2.0)
        .layout(ctx, layout);
}

impl Default for LevelPanel {
    fn default() -> Self {
        Self {
            case: 0,

            height: 0.0,
            previous_result: None,
        }
    }
}
