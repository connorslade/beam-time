use engine::{
    color::Rgb,
    drawable::{sprite::Sprite, text::Text},
    exports::{nalgebra::Vector2, winit::event::MouseButton},
    graphics_context::{Anchor, GraphicsContext},
};
use parking_lot::MutexGuard;
use thousands::Separable;

use crate::{
    app::App,
    assets::{
        BIG_RIGHT_ARROW, INFO_PANEL, LEFT_ARROW, RIGHT_ARROW, TILE_DETECTOR, TILE_EMITTER_DOWN,
        UNDEAD_FONT,
    },
    consts::layer,
    game::{beam::InnerSimulationState, board::Board, level::Level},
    util::in_bounds,
};

#[derive(Default)]
pub struct LevelPanel {
    case: usize,
}

struct UIContext {
    scale: f32,
    margin: f32,
    tile_size: f32,
    y: f32,
}

const WIDTH: usize = 6;

impl LevelPanel {
    pub fn render(
        &mut self,
        ctx: &mut GraphicsContext<App>,
        state: &App,
        board: &Board,
        sim: &MutexGuard<InnerSimulationState>,
    ) {
        let Some(level) = board.transient.level else {
            return;
        };

        let scale = state.config.ui_scale * 4.0;
        let tile_size = scale * ctx.scale_factor * 16.0;
        let margin = tile_size / 4.0;

        let mut ui = UIContext {
            scale,
            margin,
            tile_size,
            y: ctx.size().y - margin,
        };

        level_info(ctx, state, board, level, &mut ui);
        test_case(self, ctx, state, level, sim, &mut ui);
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

fn level_info(
    ctx: &mut GraphicsContext<App>,
    state: &App,
    board: &Board,
    level: &Level,
    ui: &mut UIContext,
) {
    let padding = 10.0 * state.config.ui_scale * ctx.scale_factor;

    let title = Text::new(UNDEAD_FONT, &level.name)
        .position(Vector2::new(ui.margin, ui.y), Anchor::TopLeft)
        .scale(Vector2::repeat(state.config.ui_scale * 3.0))
        .z_index(layer::UI_ELEMENT);
    ui.y -= title.size(ctx).y + padding;
    ctx.draw(title);

    let price = board
        .tiles
        .iter()
        .filter(|(pos, _)| !level.permanent.contains(pos))
        .map(|(_, tile)| tile.price())
        .sum::<u32>();
    let description = format!("${}\n\n{}", price.separate_with_commas(), level.description);
    let description = Text::new(UNDEAD_FONT, &description)
        .position(Vector2::new(ui.margin, ui.y), Anchor::TopLeft)
        .scale(Vector2::repeat(state.config.ui_scale * 2.0))
        .max_width(WIDTH as f32 * ui.tile_size - ui.margin * 2.0)
        .z_index(layer::UI_ELEMENT);
    ui.y -= description.size(ctx).y + padding;
    ctx.draw(description);
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
            .scale(Vector2::new(scale, scale), Anchor::Center)
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
                .scale(Vector2::repeat(ui.scale), Anchor::Center)
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

    let y_scale = height / 16.0;
    let x_scale = ui.scale * (WIDTH - 2) as f32;
    let x_right = ui.tile_size * WIDTH as f32;

    let base = Sprite::new(INFO_PANEL)
        .z_index(layer::UI_BACKGROUND)
        .scale(Vector2::repeat(ui.scale), Anchor::Center);

    if height > 0.0 {
        ctx.draw(
            base.clone()
                .scale(Vector2::new(ui.scale, y_scale), Anchor::Center)
                .position(Vector2::new(0.0, ctx.size().y), Anchor::TopLeft)
                .uv_offset(Vector2::new(-16, 0)),
        );

        ctx.draw(
            base.clone()
                .scale(Vector2::new(ui.scale, y_scale), Anchor::Center)
                .position(Vector2::new(x_right, ctx.size().y), Anchor::TopRight)
                .uv_offset(Vector2::new(16, 0)),
        );

        ctx.draw(
            base.clone()
                .scale(Vector2::new(x_scale, y_scale), Anchor::Center)
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
        base.scale(Vector2::new(x_scale, ui.scale), Anchor::Center)
            .position(Vector2::new(ui.tile_size, ui.y), Anchor::BottomLeft)
            .uv_offset(Vector2::new(0, 16)),
    );
}
