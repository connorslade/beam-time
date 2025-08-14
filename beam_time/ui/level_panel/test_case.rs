use parking_lot::MutexGuard;

use crate::{
    assets::{
        BIG_RIGHT_ARROW, HISTOGRAM_MARKER, LEFT_ARROW, RIGHT_ARROW, TILE_DETECTOR,
        TILE_EMITTER_DOWN, UNDEAD_FONT,
    },
    consts::spacing::MARGIN,
    ui::{components::manual_button::ManualButton, level_panel::horizontal_rule, misc::tile_label},
};
use beam_logic::{
    level::{ElementLocation, Level, LevelIo as Io, case::CasePreview},
    simulation::runtime::asynchronous::InnerAsyncSimulationState,
};
use engine::{
    color::Rgb,
    drawable::{dummy::DummyDrawable, spacer::Spacer, sprite::Sprite, text::Text},
    exports::nalgebra::Vector2,
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
            (sim_level.test_case + sim_level.test_offset) % level.tests.visible_count()
        } else {
            self.case
        };

        let case = level.tests.get_visible(case_idx);
        let Some(preview) = case.preview(level) else {
            DummyDrawable::new().layout(ctx, layout);
            return;
        };

        let scale = Vector2::repeat(4.0);
        horizontal_rule(ctx, layout);

        ColumnLayout::new(MARGIN).show(ctx, layout, |ctx, layout| {
            RowLayout::new(0.0)
                .justify(Justify::Center)
                .show(ctx, layout, |ctx, layout| {
                    if preview.elements() < 6 {
                        case_big(ctx, layout, level, &preview);
                    } else {
                        case_small(ctx, layout, level, &preview);
                    }

                    layout.nest(
                        ctx,
                        RowLayout::new(8.0)
                            .justify(Justify::Center)
                            .direction(Direction::MaxToMin),
                        |ctx, layout| {
                            let mut button =
                                |ctx: &mut _, layout: &mut RowLayout, sprite, direction: bool| {
                                    let key = memory_key!(direction);
                                    let tracker = LayoutTracker::new(key);
                                    let button = ManualButton::new(key).tracker(ctx, tracker);

                                    let disabled = (!direction && self.case == 0)
                                        || (direction
                                            && self.case + 1 == level.tests.visible_count())
                                        || is_test;

                                    let inc = (!disabled && button.pressed(ctx)) as usize;
                                    if direction {
                                        self.case += inc;
                                    } else {
                                        self.case -= inc;
                                    }

                                    let color = [0.25, 0.9, 1.0].map(Rgb::repeat)
                                        [(1 + !button.is_hovered() as usize) * !disabled as usize];

                                    Sprite::new(sprite)
                                        .scale(scale)
                                        .color(color)
                                        .tracked(tracker)
                                        .layout(ctx, layout);
                                };

                            let digits = level.tests.visible_count().ilog10() as usize + 1;
                            let width = (digits * 4 + digits - 1) as f32 * 4.0;
                            button(ctx, layout, RIGHT_ARROW, true);
                            layout.nest(
                                ctx,
                                RowLayout::new(0.0)
                                    .sized(Vector2::x() * width)
                                    .direction(Direction::MaxToMin),
                                |ctx, layout| {
                                    layout.nest(ctx, RowLayout::new(0.0), |ctx, layout| {
                                        let text =
                                            format!("{:0>width$}", case_idx + 1, width = digits);
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
                });

            if let Some(display) = &level.tests.display
                && let Some(desc) = display.descriptions.get(&(case_idx as u32))
            {
                RowLayout::new(0.0).show(ctx, layout, |ctx, layout| {
                    Spacer::new_x(2.0 * 4.0).layout(ctx, layout);
                    Text::new(UNDEAD_FONT, desc)
                        .max_width(layout.available().x)
                        .scale(Vector2::repeat(2.0))
                        .layout(ctx, layout);
                });
            }
        });
    }
}

fn case_big(
    ctx: &mut GraphicsContext,
    layout: &mut RowLayout,
    level: &Level,
    preview: &CasePreview<'_, '_>,
) {
    render_tiles(ctx, layout, 4.0, level, Io::Emitter, preview.laser());

    Sprite::new(BIG_RIGHT_ARROW)
        .scale(Vector2::repeat(4.0))
        .layout(ctx, layout);

    render_tiles(ctx, layout, 4.0, level, Io::Detector, preview.detector());
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
            render_tiles(ctx, layout, 2.0, level, Io::Emitter, preview.laser());

            Spacer::new_y(8.0).layout(ctx, layout);
            Sprite::new(HISTOGRAM_MARKER)
                .scale(Vector2::repeat(2.0))
                .layout(ctx, layout);

            render_tiles(ctx, layout, 2.0, level, Io::Detector, preview.detector());
        },
    );
}

fn render_tiles<'a, T: Layout>(
    ctx: &mut GraphicsContext,
    layout: &mut T,
    scale: f32,
    level: &Level,
    io_type: Io,
    items: impl Iterator<Item = (&'a bool, u32)>,
) {
    let tile_label_offset = Vector2::repeat(8.0 * scale);
    let tile_label = |pos| -> Box<dyn LayoutElement> {
        if let Some(label) = level.labels.get(&pos) {
            Box::new(tile_label(scale, tile_label_offset, label).z_index(1))
        } else {
            Box::new(DummyDrawable::new())
        }
    };

    let row_spacing = -2.0 * scale;
    let sprite = [TILE_DETECTOR, TILE_EMITTER_DOWN][matches!(io_type, Io::Emitter) as usize];
    let (mut column, mut row) = (ColumnLayout::new(0.0), RowLayout::new(row_spacing));

    for (idx, (&input, tile)) in items.enumerate() {
        let label = tile_label(ElementLocation::Dynamic(tile));
        let tile_sprite = Sprite::new(sprite)
            .uv_offset(Vector2::new(16 * input as i32, 0))
            .scale(Vector2::repeat(scale));
        Container::of(ctx, [Box::new(tile_sprite), label]).layout(ctx, &mut row);

        if let Some(config) = &level.tests.display {
            if config.do_space(io_type, idx) {
                Spacer::new_x(16.0 * scale).layout(ctx, &mut row);
            }

            if config.do_break(io_type, idx) {
                row.layout(ctx, &mut column);
                row = RowLayout::new(row_spacing);
            }
        }
    }

    row.layout(ctx, &mut column);
    column.layout(ctx, layout);
}
