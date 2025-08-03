use std::f32::consts::PI;

use common::misc::exp_decay;
use parking_lot::MutexGuard;
use thousands::Separable;

use crate::{
    app::App,
    assets::{COLLAPSE, UNDEAD_FONT},
    consts::layer,
    game::board::Board,
};
use beam_logic::{
    level::Level,
    simulation::{level_state::LevelResult, runtime::asynchronous::InnerAsyncSimulationState},
};
use engine::{
    drawable::{spacer::Spacer, sprite::Sprite, text::Text},
    exports::{
        nalgebra::Vector2,
        winit::{event::MouseButton, window::CursorIcon},
    },
    graphics_context::{Anchor, GraphicsContext},
    layout::{
        Direction, Justify, Layout, LayoutElement, LayoutMethods, column::ColumnLayout,
        row::RowLayout, tracker::LayoutTracker,
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

        let tile_size = 4.0 * 16.0 * ctx.scale_factor;
        let (margin, padding) = state.spacing(ctx);
        let width = tile_size * WIDTH as f32;

        // idk maybe cache or smth — not that it really matters
        let (price, tiles) = price(board, level);

        let trackers @ [base, extended] = [memory_key!(), memory_key!()].map(LayoutTracker::new);
        let tracker = trackers[level_result.is_some() as usize];

        let height = tracker
            .bounds(ctx)
            .map(|x| ctx.size().y - x.min.y)
            .unwrap_or_default()
            - ctx.input.delta_size().y;
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

                layout.nest(
                    ctx,
                    RowLayout::new(0.0).direction(Direction::MaxToMin),
                    |ctx, layout| {
                        let tracker = LayoutTracker::new(memory_key!());
                        if tracker.hovered(ctx) {
                            ctx.set_cursor(CursorIcon::Pointer);
                            self.collapsed ^= ctx.input.mouse_pressed(MouseButton::Left);
                        }

                        Sprite::new(COLLAPSE)
                            .scale(Vector2::repeat(3.0))
                            .rotate(self.collapsed as u8 as f32 * PI, Anchor::Center)
                            .tracked(tracker)
                            .layout(ctx, layout);
                        Spacer::new_x(layout.available().x).layout(ctx, layout);
                    },
                );
            },
        );

        let padding = 10.0 * ctx.scale_factor;
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
            let desired = (-bounds.height() - padding * 2.0) * self.collapsed as u8 as f32;

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
            let top = status_bounds.min.y - padding + ctx.input.delta_size().y;
            desc = desc.clip(Vector2::zeros(), Vector2::new(f32::MAX, top));
        }

        desc.tracked(tracker).layout(ctx, layout);
    }
}

// todo: Don't re-calc price every frame
fn price(board: &Board, level: &Level) -> (u32, usize) {
    let (mut price, mut count) = (0, 0);
    for (pos, tile) in board.tiles.iter() {
        if level.permanent.contains(&pos) {
            continue;
        }

        price += tile.price();
        count += 1;
    }

    (price, count)
}

fn horizontal_rule(ctx: &mut GraphicsContext, layout: &mut ColumnLayout) {
    let margin = 12.0 * ctx.scale_factor;
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
