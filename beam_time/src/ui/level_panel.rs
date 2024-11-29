use std::f32::consts::PI;

use engine::{
    color::{OkLab, Rgb},
    drawable::{sprite::Sprite, text::Text},
    exports::{
        nalgebra::{Vector2, Vector3},
        winit::event::MouseButton,
    },
    graphics_context::{Anchor, GraphicsContext},
};
use parking_lot::MutexGuard;
use thousands::Separable;

use beam_logic::{level::LevelResult, InnerSimulationState};
use crate::{
    app::App,
    assets::{
        BIG_RIGHT_ARROW, HISTOGRAM_BAR, HORIZONTAL_RULE, INFO_PANEL, LEFT_ARROW, RIGHT_ARROW,
        TILE_DETECTOR, TILE_EMITTER_DOWN, UNDEAD_FONT,
    },
    consts::layer,
    game::{
        ,
        board::Board,
        level::Level,
    },
    util::in_bounds,
};

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
        sim: &MutexGuard<InnerSimulationState>,
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
        horizontal_rule(ctx, &mut ui);
        test_case(self, ctx, state, level, sim, &mut ui);

        if let Some(result) = level_result {
            horizontal_rule(ctx, &mut ui);
            level_complete(ctx, state, result, price, &mut ui);
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

fn horizontal_rule(ctx: &mut GraphicsContext<App>, ui: &mut UIContext) {
    ui.y -= ui.margin;
    for i in 0..WIDTH {
        ctx.draw(
            Sprite::new(HORIZONTAL_RULE)
                .scale(Vector2::repeat(ui.scale))
                .position(Vector2::new(ui.tile_size * i as f32, ui.y), Anchor::TopLeft)
                .color(Rgb::repeat(0.459))
                .z_index(layer::UI_ELEMENT),
        );
    }
    ui.y -= ui.scale + ui.margin;
}

fn level_info(
    ctx: &mut GraphicsContext<App>,
    state: &App,
    level: &Level,
    price: u32,
    ui: &mut UIContext,
) {
    let title = Text::new(UNDEAD_FONT, &level.name)
        .position(Vector2::new(ui.margin, ui.y), Anchor::TopLeft)
        .scale(Vector2::repeat(state.config.ui_scale * 3.0))
        .z_index(layer::UI_ELEMENT);
    ui.y -= title.size(ctx).y + ui.padding;
    ctx.draw(title);

    let description = format!("${}\n\n{}", price.separate_with_commas(), level.description);
    let description = Text::new(UNDEAD_FONT, &description)
        .position(Vector2::new(ui.margin, ui.y), Anchor::TopLeft)
        .scale(Vector2::repeat(state.config.ui_scale * 2.0))
        .max_width(WIDTH as f32 * ui.tile_size - ui.margin * 2.0)
        .z_index(layer::UI_ELEMENT);
    ui.y -= description.size(ctx).y + ui.padding;
    ctx.draw(description);
}

fn level_complete(
    ctx: &mut GraphicsContext<App>,
    state: &App,
    result: &LevelResult,
    price: u32,
    ui: &mut UIContext,
) {
    let center_x = (WIDTH as f32 * ui.tile_size) / 2.0;

    let title = Text::new(UNDEAD_FONT, "Level Complete!")
        .position(Vector2::new(center_x, ui.y), Anchor::TopCenter)
        .scale(Vector2::repeat(state.config.ui_scale * 3.0))
        .z_index(layer::UI_ELEMENT);
    ui.y -= title.size(ctx).y + ui.scale + ui.padding;

    let now = state.start.elapsed().as_secs_f32();
    ctx.draw_callback(title, |sprites| {
        let count = sprites.len();
        for (idx, sprite) in sprites.iter_mut().enumerate() {
            let t = idx as f32 / count as f32;
            let color = OkLab::new(0.8, 0.1893, 0.0)
                .hue_shift(t * 2.0 * PI - now * 2.0)
                .to_lrgb();
            sprite.color = Vector3::new(color.r, color.g, color.b).map(|x| x as f32 / 255.0);

            let offset = (t * 2.0 * PI - now * 6.0).sin() * ui.scale;
            sprite.points.iter_mut().for_each(|point| point.y += offset);
        }
    });

    let text = format!(
        "Nice work! Your solution costs ${} and has a total latency of {} ticks.",
        price.separate_with_commas(),
        result.latency
    );
    let text = Text::new(UNDEAD_FONT, &text)
        .position(Vector2::new(ui.margin, ui.y), Anchor::TopLeft)
        .scale(Vector2::repeat(state.config.ui_scale * 2.0))
        .max_width(WIDTH as f32 * ui.tile_size - ui.margin * 2.0)
        .z_index(layer::UI_ELEMENT);
    ui.y -= text.size(ctx).y + ui.padding * 2.0;
    ctx.draw(text);

    for (i, text) in ["Cost", "Latency"].iter().enumerate() {
        let pos = Vector2::new(
            WIDTH as f32 * ui.tile_size * (1.0 + 2.0 * i as f32) / 4.0,
            ui.y,
        );
        let text = Text::new(UNDEAD_FONT, text)
            .position(pos, Anchor::TopCenter)
            .scale(Vector2::repeat(state.config.ui_scale * 2.0))
            .z_index(layer::UI_ELEMENT);
        let offset = text.size(ctx).y + ui.padding * 2.0;

        let hist_pos = Vector2::new(ui.tile_size * WIDTH as f32 / 2.0 * i as f32, ui.y - offset);
        let height = histogram(ctx, state, ui, hist_pos, ui.tile_size, 75.0);
        ui.y -= (offset + height) * (i == 1) as u8 as f32;

        ctx.draw(text);
    }
}

fn histogram(
    ctx: &mut GraphicsContext<App>,
    state: &App,
    ui: &mut UIContext,
    base: Vector2<f32>,
    height: f32,

    actual: f32,
) -> f32 {
    const BIN_COUNT: usize = 12;

    // normally distriubted example data
    let data = [
        62, 51, 49, 61, 81, 44, 33, 48, 53, 45, 72, 42, 65, 29, 66, 27, 33, 59, 16, 22, 43, 60, 44,
        61, 71, 47, 31, 21, 45, 33, 45, 41, 70, 66, 52, 66, 58, 40, 69, 41, 76, 100, 44, 40, 78,
        53, 15, 57, 34, 70, 43, 94, 22, 21, 44, 23, 47, 61, 59, 17, 25, 45, 14, 61, 57, 75, 17, 10,
        41, 41, 33, 51, 58, 46, 48, 58, 54, 49, 23, 57, 59, 41, 48, 18, 59, 46, 91, 21, 33, 40, 62,
        47, 41, 51, 83, 58, 46, 53, 73, 63, 43, 44, 58, 35, 39, 29, 55, 94, 45, 81, 44, 34, 61, 73,
        70, 14, 37, 44, 50, 55, 56, 48, 17, 47, 73, 90, 24, 68, 39, 44, 88, 83, 62, 76, 58, 21, 65,
        26, 36, 43, 72, 43, 51, 39, 63, 58, 43, 41, 65, 73, 77, 53, 65, 91, 86, 60, 86, 1, 62, 33,
        49, 74, 59, 78, 34, 51, 36, 74, 78, 25, 55, 72, 67, 63, 73, 23, 55, 50, 30, 58, 47, 45, 57,
        16, 58, 63, 57, 77, 18, 83, 26, 45, 42, 75, 78, 30, 34, 52, 59, 20, 71, 64, 86, 23, 66, 69,
        27, 27, 30, 37, 37, 63, 35, 24, 30, 54, 30, 38, 30, 29, 77, 77, 55, 4, 67, 25, 38, 25, 28,
        40, 51, 30, 51, 78, 37, 43, 72, 64, 49, 56, 53, 79, 32, 65, 19, 53, 13, 67, 46, 53, 75, 30,
        39, 57, 72, 47, 50, 33, 61, 74, 6, 49, 54, 31, 80, 44, 13, 64, 62, 11, 68, 61, 85, 49, 38,
        43, 48, 52, 50, 19, 33, 46, 35, 41, 55, 74, 10, 43, 43, 36, 56, 42, 91, 31, 18, 61, 55, 35,
        21, 41,
    ];

    // todo: remove outliers?

    let max = *data.iter().max().unwrap();
    let bin_width = max as f32 / BIN_COUNT as f32;

    let mut bins = vec![0; BIN_COUNT];
    for point in data {
        let bin = (point as f32 / bin_width) as usize;
        bins[bin.min(BIN_COUNT - 1)] += 1;
    }

    let max_count = *bins.iter().max().unwrap();

    // ^ Generate histogram

    let bars = bins
        .iter()
        .enumerate()
        .map(|(idx, &count)| {
            base + Vector2::new(
                ui.tile_size / 4.0 * (idx as f32 + 1.0),
                ((count as f32 / max_count as f32) - 1.0) * height,
            )
        })
        .collect::<Vec<_>>();

    for &pos in bars.iter() {
        ctx.draw(
            Sprite::new(HISTOGRAM_BAR)
                .position(pos, Anchor::TopLeft)
                .scale(Vector2::repeat(state.config.ui_scale * 4.0))
                .z_index(layer::UI_ELEMENT),
        );
    }

    let scale = ctx.scale_factor * state.config.ui_scale * 4.0;
    for pos in bars.windows(2).filter(|x| x[0].y != x[1].y) {
        let (y1, y2) = (pos[0].y, pos[1].y);
        let height = -(y1 - y2).abs() - scale;

        let pos = Vector2::new(pos[1].x, y1.min(y2) - scale);
        ctx.draw(
            Sprite::new(HISTOGRAM_BAR)
                .position(pos, Anchor::TopCenter)
                .scale(Vector2::new(1.0, height / ctx.scale_factor) * state.config.ui_scale)
                .z_index(layer::UI_ELEMENT),
        );
    }

    let text = Text::new(UNDEAD_FONT, "0")
        .position(
            base + Vector2::new(ui.tile_size / 4.0, -height - ui.padding),
            Anchor::TopLeft,
        )
        .scale(Vector2::repeat(state.config.ui_scale * 2.0))
        .z_index(layer::UI_ELEMENT);
    let text_height = text.size(ctx).y + ui.padding;
    ctx.draw(text);

    ctx.draw(
        Text::new(UNDEAD_FONT, max.to_string().as_str())
            .position(
                base + Vector2::new(
                    ui.tile_size / 4.0 * (1.0 + BIN_COUNT as f32),
                    -height - ui.padding,
                ),
                Anchor::TopRight,
            )
            .scale(Vector2::repeat(state.config.ui_scale * 2.0))
            .z_index(layer::UI_ELEMENT),
    );

    let t = actual / max as f32;
    ctx.draw(
        Text::new(UNDEAD_FONT, actual.to_string().as_str())
            .scale(Vector2::repeat(state.config.ui_scale * 2.0))
            .position(
                base + Vector2::new(
                    ui.tile_size / 4.0 * (1.0 + t * BIN_COUNT as f32),
                    -height - ui.padding,
                ),
                Anchor::TopCenter,
            )
            .z_index(layer::UI_ELEMENT),
    );

    height + text_height
}

fn test_case(
    panel: &mut LevelPanel,
    ctx: &mut GraphicsContext<App>,
    state: &App,
    level: &Level,
    sim: &MutexGuard<InnerSimulationState>,
    ui: &mut UIContext,
) {
    let sim_level = sim.beam.as_ref().and_then(|x| x.level.as_ref());
    let is_test = sim_level.is_some();
    let case_idx = if let Some(level) = sim_level {
        level.test_case
    } else {
        panel.case
    };

    let case = &level.tests.cases[case_idx];
    let case_elements = case.lasers.len() + case.detectors[0].len() + 1;
    let (mut scale, mut tile_size, mut arrow_size) = (
        ui.scale,
        ui.tile_size,
        11.0 * 4.0 * ctx.scale_factor * state.config.ui_scale,
    );

    if case_elements + 1 > WIDTH {
        scale /= 2.0;
        tile_size /= 2.0;
        arrow_size /= 2.0;
    }

    let case_tile = |texture| {
        Sprite::new(texture)
            .scale(Vector2::new(scale, scale))
            .z_index(layer::UI_ELEMENT)
    };

    let mut i = 0;
    for &input in &case.lasers {
        let pos = Vector2::new(ui.margin + i as f32 * tile_size, ui.y);
        ctx.draw(
            case_tile(TILE_EMITTER_DOWN)
                .uv_offset(Vector2::new(-16 * input as i32, 0))
                .position(pos, Anchor::TopLeft),
        );
        i += 1;
    }

    ctx.draw(case_tile(BIG_RIGHT_ARROW).position(
        Vector2::new(ui.margin + i as f32 * tile_size, ui.y - tile_size / 2.0),
        Anchor::CenterLeft,
    ));

    for &input in &case.detectors[0] {
        let pos = Vector2::new(ui.margin + i as f32 * tile_size + arrow_size, ui.y);
        ctx.draw(
            case_tile(TILE_DETECTOR)
                .uv_offset(Vector2::new(16 * input as i32, 0))
                .position(pos, Anchor::TopLeft),
        );
        i += 1;
    }

    let mut pos = Vector2::new(
        ui.tile_size * WIDTH as f32 - ui.margin,
        ui.y - tile_size / 2.0,
    );
    let button_width = 4.0 * 3.0 * ctx.scale_factor * state.config.ui_scale;
    let button_padding = 4.0 * 2.0 * ctx.scale_factor * state.config.ui_scale;

    let mut case_button =
        |ctx: &mut GraphicsContext<App>, dir: bool, pos: Vector2<f32>| -> Sprite {
            let texture = if dir { RIGHT_ARROW } else { LEFT_ARROW };
            let mut case = case_tile(texture)
                .scale(Vector2::repeat(ui.scale))
                .position(pos, Anchor::CenterRight);

            if (!dir && panel.case == 0)
                || (dir && panel.case + 1 == level.tests.cases.len())
                || is_test
            {
                case = case.color(Rgb::repeat(0.25));
            } else if case.is_hovered(ctx) {
                case = case.color(Rgb::repeat(0.9));
                let d_case = ctx.input.mouse_pressed(MouseButton::Left) as usize;
                if dir {
                    panel.case += d_case;
                } else {
                    panel.case -= d_case;
                }
            }

            case
        };

    let right_button = case_button(ctx, true, pos);
    pos.x -= button_width + button_padding;

    let index_text = (case_idx + 1).to_string();
    let index = Text::new(UNDEAD_FONT, &index_text)
        .position(pos, Anchor::CenterRight)
        .scale(Vector2::repeat(ui.scale))
        .z_index(layer::UI_ELEMENT);
    pos.x -= index.size(ctx).x + button_padding;

    let left_button = case_button(ctx, false, pos);

    ctx.draw(left_button);
    ctx.draw(index);
    ctx.draw(right_button);

    ui.y -= tile_size;
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
        ctx.draw(
            base.clone()
                .scale(Vector2::new(ui.scale, y_scale))
                .position(Vector2::new(0.0, ctx.size().y), Anchor::TopLeft)
                .uv_offset(Vector2::new(-16, 0)),
        );

        ctx.draw(
            base.clone()
                .scale(Vector2::new(ui.scale, y_scale))
                .position(Vector2::new(x_right, ctx.size().y), Anchor::TopRight)
                .uv_offset(Vector2::new(16, 0)),
        );

        ctx.draw(
            base.clone()
                .scale(Vector2::new(x_scale, y_scale))
                .position(Vector2::new(ui.tile_size, ctx.size().y), Anchor::TopLeft),
        );
    }

    ctx.draw(
        base.clone()
            .position(Vector2::new(0.0, ui.y), Anchor::BottomLeft)
            .uv_offset(Vector2::new(-16, 16)),
    );

    ctx.draw(
        base.clone()
            .position(Vector2::new(x_right, ui.y), Anchor::BottomRight)
            .uv_offset(Vector2::new(16, 16)),
    );

    ctx.draw(
        base.scale(Vector2::new(x_scale, ui.scale))
            .position(Vector2::new(ui.tile_size, ui.y), Anchor::BottomLeft)
            .uv_offset(Vector2::new(0, 16)),
    );
}
