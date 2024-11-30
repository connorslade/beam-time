use parking_lot::MutexGuard;

use crate::{
    app::App,
    assets::{
        BIG_RIGHT_ARROW, LEFT_ARROW, RIGHT_ARROW, TILE_DETECTOR, TILE_EMITTER_DOWN, UNDEAD_FONT,
    },
    consts::layer,
};
use beam_logic::{level::Level, simulation::runtime::asynchronous::InnerAsyncSimulationState};
use engine::{
    color::Rgb,
    drawable::{sprite::Sprite, text::Text},
    exports::{nalgebra::Vector2, winit::event::MouseButton},
    graphics_context::{Anchor, GraphicsContext},
};

use super::{LevelPanel, UIContext, WIDTH};

pub fn test_case(
    panel: &mut LevelPanel,
    ctx: &mut GraphicsContext<App>,
    state: &App,
    level: &Level,
    sim: &MutexGuard<InnerAsyncSimulationState>,
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

    ctx.draw([left_button, right_button]);
    ctx.draw(index);

    ui.y -= tile_size;
}
