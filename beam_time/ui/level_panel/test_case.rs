use parking_lot::MutexGuard;

use crate::{
    assets::{
        BIG_RIGHT_ARROW, HISTOGRAM_MARKER, LEFT_ARROW, RIGHT_ARROW, TILE_DETECTOR,
        TILE_EMITTER_DOWN, UNDEAD_FONT,
    },
    ui::{level_panel::horizontal_rule, misc::tile_label},
};
use beam_logic::{
    level::{ElementLocation, Level, case::CasePreview},
    simulation::runtime::asynchronous::InnerAsyncSimulationState,
};
use engine::{
    assets::SpriteRef,
    color::Rgb,
    drawable::{dummy::DummyDrawable, spacer::Spacer, sprite::Sprite, text::Text},
    exports::{nalgebra::Vector2, winit::event::MouseButton},
    graphics_context::GraphicsContext,
    layout::{
        Direction, Justify, Layout, LayoutElement, LayoutMethods, column::ColumnLayout,
        container::Container, row::RowLayout, tracker::LayoutTracker,
    },
    memory_key,
};

use super::LevelPanel;

impl LevelPanel {
    pub(super) fn test_case(
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
            DummyDrawable::new().layout(ctx, layout);
            return;
        };

        let scale = Vector2::repeat(4.0);
        horizontal_rule(ctx, layout);

        layout.nest(
            ctx,
            RowLayout::new(0.0).justify(Justify::Center),
            |ctx, layout| {
                if preview.elements() < 6 {
                    case_big(ctx, layout, level, &preview);
                } else {
                    case_small(ctx, layout, level, &preview);
                }

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

                        let digits = preview.elements().ilog10() as usize + 1;
                        let width = (digits * 4 + digits - 1) as f32 * 4.0 * ctx.scale_factor;
                        button(ctx, layout, RIGHT_ARROW, true);
                        layout.nest(
                            ctx,
                            RowLayout::new(0.0)
                                .sized(Vector2::x() * width)
                                .direction(Direction::MaxToMin),
                            |ctx, layout| {
                                layout.nest(ctx, RowLayout::new(0.0), |ctx, layout| {
                                    let text = format!("{:0>width$}", case_idx + 1, width = digits);
                                    Text::new(UNDEAD_FONT, text)
                                        .scale(scale)
                                        .layout(ctx, layout);
                                    let half_width = layout.available().x / 2.0;
                                    Spacer::new_x(half_width).layout(ctx, layout);
                                });
                                Spacer::new_x(layout.available().x).layout(ctx, layout);
                            },
                        );
                        button(ctx, layout, LEFT_ARROW, false);
                        Spacer::new_x(layout.available().x).layout(ctx, layout);
                    },
                );
            },
        );
    }
}

fn case_big(
    ctx: &mut GraphicsContext,
    layout: &mut RowLayout,
    level: &Level,
    preview: &CasePreview<'_, '_>,
) {
    render_tiles(ctx, layout, 4.0, level, TILE_EMITTER_DOWN, preview.laser());

    Sprite::new(BIG_RIGHT_ARROW)
        .scale(Vector2::repeat(4.0))
        .layout(ctx, layout);

    render_tiles(ctx, layout, 4.0, level, TILE_DETECTOR, preview.detector());
}

fn case_small(
    ctx: &mut GraphicsContext,
    layout: &mut RowLayout,
    level: &Level,
    preview: &CasePreview<'_, '_>,
) {
    layout.nest(
        ctx,
        ColumnLayout::new(0.0).justify(Justify::Center),
        |ctx, layout| {
            layout.nest(ctx, RowLayout::new(0.0), |ctx, layout| {
                render_tiles(ctx, layout, 2.0, level, TILE_EMITTER_DOWN, preview.laser());
            });

            Spacer::new_y(8.0 * ctx.scale_factor).layout(ctx, layout);
            Sprite::new(HISTOGRAM_MARKER)
                .scale(Vector2::repeat(2.0))
                .layout(ctx, layout);

            layout.nest(ctx, RowLayout::new(0.0), |ctx, layout| {
                render_tiles(ctx, layout, 2.0, level, TILE_DETECTOR, preview.detector());
            });
        },
    );
}

fn render_tiles<'a>(
    ctx: &mut GraphicsContext,
    layout: &mut RowLayout,
    scale: f32,
    level: &Level,
    sprite: SpriteRef,
    items: impl Iterator<Item = (&'a bool, u32)>,
) {
    let tile_label_offset = Vector2::repeat(8.0 * scale * ctx.scale_factor);
    let tile_label = |ctx: &mut GraphicsContext, pos| -> Box<dyn LayoutElement> {
        if let Some(label) = level.labels.get(&pos) {
            Box::new(tile_label(ctx, scale, tile_label_offset, label).z_index(1))
        } else {
            Box::new(DummyDrawable::new())
        }
    };

    for (&input, tile) in items {
        let label = tile_label(ctx, ElementLocation::Dynamic(tile));
        let tile_sprite = Sprite::new(sprite)
            .uv_offset(Vector2::new(16 * input as i32, 0))
            .scale(Vector2::repeat(scale));
        Container::of(ctx, [Box::new(tile_sprite), label]).layout(ctx, layout);
    }
}
