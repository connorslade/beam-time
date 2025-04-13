use parking_lot::MutexGuard;
use thousands::Separable;

use crate::{
    app::App,
    assets::{
        BIG_RIGHT_ARROW, LEFT_ARROW, RIGHT_ARROW, TILE_DETECTOR, TILE_EMITTER_DOWN, UNDEAD_FONT,
    },
    consts::layer,
    game::board::Board,
    ui::misc::tile_label,
};
use beam_logic::{
    level::{ElementLocation, Level},
    simulation::{level_state::LevelResult, runtime::asynchronous::InnerAsyncSimulationState},
};
use engine::{
    assets::SpriteRef,
    color::Rgb,
    drawable::{dummy::DummyDrawable, spacer::Spacer, sprite::Sprite, text::Text},
    exports::{nalgebra::Vector2, winit::event::MouseButton},
    graphics_context::{Anchor, GraphicsContext},
    layout::{
        column::ColumnLayout, container::Container, row::RowLayout, tracker::LayoutTracker,
        Direction, Justify, Layout, LayoutElement, LayoutMethods,
    },
    memory_key,
};

use super::components::{
    horizontal_rule::HorizontalRule,
    modal::{Modal, ModalSides},
};

pub struct LevelPanel {
    pub case: usize,

    offset: f32,
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

        Modal::new(Vector2::new(width, 500.0))
            .position(Vector2::y() * ctx.size().y, Anchor::TopLeft)
            .layer(layer::UI_BACKGROUND)
            .sides(ModalSides::BOTTOM | ModalSides::RIGHT)
            .margin(margin)
            .popup(false)
            .draw(ctx, |ctx, layout| {
                layout.nest(ctx, ColumnLayout::new(padding), |ctx, layout| {
                    self.level_info(ctx, layout, level, price);
                    self.test_case(ctx, layout, level, sim);
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

    fn test_case(
        &mut self,
        ctx: &mut GraphicsContext,
        layout: &mut ColumnLayout,
        level: &Level,
        sim: &MutexGuard<InnerAsyncSimulationState>,
    ) {
        let sim_level = sim.beam.as_ref().and_then(|x| x.level.as_ref());
        let is_test = sim_level.is_some();
        let case_idx = if let Some(sim_level) = sim_level {
            (sim_level.test_case + sim_level.test_offset) % level.tests.cases.len()
        } else {
            self.case
        };

        let case = &level.tests.cases[case_idx];
        let Some(preview) = case.preview(level) else {
            return;
        };

        let scale = Vector2::repeat(4.0);
        horizontal_rule(ctx, layout);

        layout.nest(
            ctx,
            RowLayout::new(0.0).justify(Justify::Center),
            |ctx, layout| {
                render_tiles(ctx, layout, level, TILE_EMITTER_DOWN, preview.laser());

                Sprite::new(BIG_RIGHT_ARROW)
                    .scale(scale)
                    .layout(ctx, layout);

                render_tiles(ctx, layout, level, TILE_DETECTOR, preview.detector());

                layout.nest(
                    ctx,
                    RowLayout::new(8.0 * ctx.scale_factor)
                        .justify(Justify::Center)
                        .direction(Direction::MaxToMin),
                    |ctx, layout| {
                        let mut button =
                            |ctx: &mut _, layout: &mut RowLayout, sprite, direction: bool| {
                                let tracker = LayoutTracker::new(memory_key!(direction));
                                let hovered = tracker.hovered(ctx);
                                let clicking = ctx.input.mouse_pressed(MouseButton::Left);

                                let disabled = (!direction && self.case == 0)
                                    || (direction && self.case + 1 == level.tests.cases.len())
                                    || is_test;

                                let inc = (hovered && !disabled && clicking) as usize;
                                if direction {
                                    self.case += inc;
                                } else {
                                    self.case -= inc;
                                }

                                let color = [0.25, 0.9, 1.0].map(Rgb::repeat)
                                    [(1 + !hovered as usize) * !disabled as usize];

                                Sprite::new(sprite)
                                    .scale(scale)
                                    .color(color)
                                    .tracked(tracker)
                                    .layout(ctx, layout);
                            };

                        button(ctx, layout, RIGHT_ARROW, true);
                        Text::new(UNDEAD_FONT, (case_idx + 1).to_string())
                            .scale(scale)
                            .layout(ctx, layout);
                        button(ctx, layout, LEFT_ARROW, false);
                        Spacer::new_x(layout.available().x).layout(ctx, layout);
                    },
                );
            },
        );
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
        .margin(margin)
        .layout(ctx, layout);
}

fn render_tiles<'a>(
    ctx: &mut GraphicsContext,
    layout: &mut RowLayout,
    level: &Level,
    sprite: SpriteRef,
    items: impl Iterator<Item = (&'a bool, &'a ElementLocation)>,
) {
    let tile_label_offset = Vector2::repeat(32.0 * ctx.scale_factor);
    let tile_label = |ctx: &mut GraphicsContext, pos| -> Box<dyn LayoutElement> {
        if let Some(label) = level.labels.get(&pos) {
            Box::new(tile_label(ctx, 4.0, tile_label_offset, label).z_index(1))
        } else {
            Box::new(DummyDrawable)
        }
    };

    for (&input, tile) in items {
        let label = tile_label(ctx, tile.into_pos());
        let tile_sprite = Sprite::new(sprite)
            .uv_offset(Vector2::new(16 * input as i32, 0))
            .scale(Vector2::repeat(4.0));
        Container::of(ctx, [label, Box::new(tile_sprite)]).layout(ctx, layout);
    }
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
