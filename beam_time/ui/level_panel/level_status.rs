use std::f32::consts::PI;

use rand::{Rng, rng};
use thousands::Separable;

use crate::{app::App, assets::UNDEAD_FONT, consts::color, ui::components::histogram::Histogram};
use beam_logic::{level::Level, simulation::level_state::LevelResult};
use engine::{
    color::{OkLab, Rgb},
    drawable::{spacer::Spacer, text::Text},
    exports::nalgebra::Vector2,
    graphics_context::GraphicsContext,
    layout::{
        Direction, Justify, Layout, LayoutElement, LayoutMethods, column::ColumnLayout,
        container::Container, row::RowLayout,
    },
    memory_key,
};

use super::{LevelPanel, horizontal_rule};

const CONGRATS: &[&str] = &[
    "Nice work!",
    "Amazing!",
    "Excellent!",
    "Great job!",
    "Well done!",
    "Fantastic!",
    "Impressive!",
    "Brilliant!",
];

impl LevelPanel {
    pub(super) fn level_status(
        &mut self,
        ctx: &mut GraphicsContext,
        state: &App,
        layout: &mut ColumnLayout,
        level: &Level,
        level_result: &Option<LevelResult>,
        price: u32,
    ) {
        if let Some((result, price)) = level_result.map(|x| (x, price)).or(self.previous_result) {
            self.previous_result = Some((result, price));
            horizontal_rule(ctx, layout);

            match result {
                LevelResult::Success { latency } => {
                    success(ctx, state, layout, level, (price, latency))
                }
                LevelResult::Failed { case } => {
                    let idx = level.tests.visible_index(case) + 1;
                    failed(ctx, layout, idx)
                }
                LevelResult::OutOfTime => unreachable!(),
            }
        }
    }
}

fn success(
    ctx: &mut GraphicsContext,
    state: &App,
    layout: &mut ColumnLayout,
    level: &Level,
    (price, latency): (u32, u32),
) {
    let now = state.start.elapsed().as_secs_f32();
    let congrat = *ctx
        .memory
        .get_or_insert_with(memory_key!(), || rng().random_range(0..CONGRATS.len()));
    let text = format!(
        "{} Your solution costs ${} and has a total latency of {latency} ticks.",
        CONGRATS[congrat],
        price.separate_with_commas()
    );

    layout.nest(
        ctx,
        layout.clone().justify(Justify::Center),
        |ctx, layout| {
            let title = Text::new(UNDEAD_FONT, "Level Complete").scale(Vector2::repeat(3.0));
            Container::of(ctx, [Box::new(title) as Box<dyn LayoutElement>])
                .callback(move |sprites, _polygons| {
                    let count = sprites.len();
                    for (idx, sprite) in sprites.iter_mut().enumerate() {
                        let t = (idx / 2) as f32 / (count / 2) as f32;
                        let color = OkLab::new(0.8, 0.1893, 0.0)
                            .hue_shift(t * 2.0 * PI - now * 2.0)
                            .to_lrgb();
                        sprite.color *=
                            Rgb::new(color.r, color.g, color.b).map(|x| x as f32 / 255.0);

                        let offset = (t * 2.0 * PI - now * 6.0).sin() * 4.0;
                        sprite.points.iter_mut().for_each(|point| point.y += offset);
                    }
                })
                .layout(ctx, layout);
            Spacer::new_y(5.0).layout(ctx, layout);

            Text::new(UNDEAD_FONT, text)
                .scale(Vector2::repeat(2.0))
                .max_width(layout.available().x)
                .layout(ctx, layout);

            Spacer::new_y(8.0).layout(ctx, layout);
            let Some(hist_data) = state.leaderboard.get_results(level.id) else {
                Text::new(UNDEAD_FONT, "Failed to load global leaderboard.")
                    .scale(Vector2::repeat(2.0))
                    .color(color::ERROR)
                    .layout(ctx, layout);
                Spacer::new_y(4.0).layout(ctx, layout);
                return;
            };

            layout.nest(ctx, RowLayout::new(0.0), |ctx, layout| {
                Histogram::new(hist_data.cost)
                    .real(price)
                    .title("Cost")
                    .layout(ctx, layout);
                layout.nest(
                    ctx,
                    layout.clone().direction(Direction::MaxToMin),
                    |ctx, layout| {
                        Histogram::new(hist_data.latency)
                            .real(latency)
                            .title("Latency")
                            .layout(ctx, layout);
                        Spacer::new_x(layout.available().x).layout(ctx, layout);
                    },
                );
            });
        },
    );
}

fn failed(ctx: &mut GraphicsContext, layout: &mut ColumnLayout, case: usize) {
    const MESSAGE: &str =
        "Check the board to see what went wrong, make your fixes, and re-run the tests.";
    layout.nest(
        ctx,
        layout.clone().justify(Justify::Center),
        |ctx, layout| {
            Text::new(UNDEAD_FONT, "Level Failed...")
                .scale(Vector2::repeat(3.0))
                .color(Rgb::hex(0xe43636))
                .layout(ctx, layout);

            let text = format!("Looks like you failed test case {case}. {MESSAGE}");
            Text::new(UNDEAD_FONT, text)
                .scale(Vector2::repeat(2.0))
                .max_width(layout.available().x)
                .layout(ctx, layout);
        },
    );
}
