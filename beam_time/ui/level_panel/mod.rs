use std::f32::consts::PI;

use common::misc::exp_decay;
use parking_lot::MutexGuard;
use thousands::Separable;

use crate::{
    app::App,
    assets::{COLLAPSE, UNDEAD_FONT},
    consts::{
        layer,
        spacing::{MARGIN, PADDING},
    },
    game::board::Board,
    ui::components::button::{ButtonEffects, ButtonExt},
};
use beam_logic::{
    level::Level,
    misc::price,
    simulation::{level_state::LevelResult, runtime::asynchronous::InnerAsyncSimulationState},
};
use engine::{
    drawable::{Anchor, dummy::DummyDrawable, spacer::Spacer, sprite::Sprite, text::Text},
    exports::nalgebra::Vector2,
    graphics_context::GraphicsContext,
    layout::{
        Direction, Justify, Layout, LayoutElement, LayoutMethods, column::ColumnLayout,
        convenience::NoPaddingExt, row::RowLayout, tracker::LayoutTracker,
    },
    memory_key,
};

use super::components::{
    horizontal_rule::Rule,
    modal::{Modal, ModalSides},
};

mod level_status;
mod test_case;

pub struct LevelPanel {
    pub case: usize,

    collapsed: bool,
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

        let tile_size = 4.0 * 16.0;
        let width = tile_size * WIDTH as f32;

        // idk maybe cache or smth — not that it really matters
        let (price, tiles) = price(&board.tiles, level);

        let trackers @ [base, extended] = [memory_key!(), memory_key!()].map(LayoutTracker::new);
        let tracker = trackers[level_result.is_some() as usize];

        let height = tracker
            .bounds(ctx)
            .map(|x| ctx.size().y - x.min.y)
            .unwrap_or_default()
            - ctx.window.delta_size().y;
        self.height = exp_decay(self.height, height, 10.0, ctx.delta_time);

        let position = Vector2::new(4.0, ctx.size().y);
        let dummy = || DummyDrawable::new().no_padding();
        Modal::new(Vector2::new(width, self.height + PADDING))
            .position(position, Anchor::TopLeft)
            .layer(layer::UI_BACKGROUND)
            .sides(ModalSides::LEFT | ModalSides::BOTTOM | ModalSides::RIGHT)
            .margin(MARGIN)
            .popup(false)
            .draw(ctx, |ctx, layout| {
                layout.nest(ctx, ColumnLayout::new(PADDING), |ctx, layout| {
                    self.level_info(ctx, layout, level, price, tiles);
                    self.test_case(ctx, layout, level, sim);
                    dummy().tracked(base).layout(ctx, layout);
                    self.level_status(ctx, state, layout, level, level_result, price);
                    dummy().tracked(extended).layout(ctx, layout);
                });
            });
    }

    // all this code just to make the description collapsible...
    // probably not worth it
    fn level_info(
        &mut self,
        ctx: &mut GraphicsContext,
        layout: &mut ColumnLayout,
        level: &Level,
        price: u32,
        tiles: usize,
    ) {
        layout.nest(
            ctx,
            RowLayout::new(0.0).justify(Justify::Center),
            |ctx, layout| {
                Text::new(UNDEAD_FONT, &level.name)
                    .scale(Vector2::repeat(3.0))
                    .layout(ctx, layout);

                RowLayout::new(PADDING)
                    .justify(Justify::Center)
                    .direction(Direction::MaxToMin)
                    .show(ctx, layout, |ctx, layout| {
                        Sprite::new(COLLAPSE)
                            .scale(Vector2::repeat(3.0))
                            .rotate(self.collapsed as u8 as f32 * PI, Anchor::Center)
                            .button(memory_key!())
                            .effects(ButtonEffects::empty())
                            .on_click(ctx, || self.collapsed ^= true)
                            .layout(ctx, layout);
                        Spacer::new_x(layout.available().x).layout(ctx, layout);
                    });
            },
        );

        let status_tracker = LayoutTracker::new(memory_key!());
        let status_text = format!(
            "${} • {tiles} tile{}",
            price.separate_with_commas(),
            if tiles == 1 { "" } else { "s" }
        );
        Text::new(UNDEAD_FONT, status_text)
            .scale(Vector2::repeat(2.0))
            .tracked(status_tracker)
            .layout(ctx, layout);

        let tracker = LayoutTracker::new(memory_key!());
        if let Some(bounds) = tracker.bounds(ctx) {
            let desired = (-bounds.height() - PADDING * 2.0) * self.collapsed as u8 as f32;

            let offset = ctx.memory.get_or_insert(memory_key!(), 0.0);
            let last_offset = *offset;
            *offset = exp_decay(*offset, desired, 10.0, ctx.delta_time);
            let delta_offset = *offset - last_offset;

            self.height += delta_offset;
            Spacer::new_y(*offset).layout(ctx, layout);
        }

        let mut desc = Text::new(UNDEAD_FONT, &level.description)
            .max_width(layout.available().x)
            .scale(Vector2::repeat(2.0));

        if let Some(status_bounds) = status_tracker.bounds(ctx) {
            let top = status_bounds.min.y - PADDING + ctx.window.delta_size().y;
            desc = desc.clip(Vector2::zeros(), Vector2::new(f32::MAX, top));
        }

        desc.tracked(tracker).layout(ctx, layout);
    }
}

fn horizontal_rule(ctx: &mut GraphicsContext, layout: &mut ColumnLayout) {
    let margin = 12.0;
    Rule::horizontal(layout.available().x + margin * 2.0)
        .position(Vector2::x() * -margin)
        .margin(margin / 2.0)
        .layout(ctx, layout);
}

impl Default for LevelPanel {
    fn default() -> Self {
        Self {
            case: 0,

            collapsed: false,
            height: 0.0,
            previous_result: None,
        }
    }
}
