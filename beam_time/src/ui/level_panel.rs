use engine::{
    color::Rgb,
    drawable::{sprite::Sprite, text::Text},
    exports::{nalgebra::Vector2, winit::event::MouseButton},
    graphics_context::{Anchor, GraphicsContext},
};

use crate::{
    app::App,
    assets::{
        BIG_RIGHT_ARROW, INFO_PANEL, LEFT_ARROW, RIGHT_ARROW, TILE_DETECTOR, TILE_EMITTER_DOWN,
        UNDEAD_FONT,
    },
    consts::layer,
    game::{board::Board, level::Level},
};

#[derive(Default)]
pub struct LevelPanel {
    case: usize,
}

struct UIContext<'a> {
    scale: f32,
    margin: f32,
    tile_size: f32,
    y: &'a mut f32,
}

const WIDTH: usize = 6;

impl LevelPanel {
    pub fn render(&mut self, ctx: &mut GraphicsContext<App>, state: &App, board: &Board) {
        let Some(level) = board.transient.level else {
            return;
        };

        let scale = state.config.ui_scale * 4.0;
        let tile_size = scale * ctx.scale_factor * 16.0;

        // Render text
        let margin = tile_size / 4.0;
        let padding = 10.0 * state.config.ui_scale * ctx.scale_factor;
        let mut y = ctx.size().y - margin;

        let title = Text::new(UNDEAD_FONT, &level.name)
            .position(Vector2::new(margin, y), Anchor::TopLeft)
            .scale(Vector2::repeat(state.config.ui_scale * 3.0))
            .z_index(layer::UI_ELEMENT);
        y -= title.size(ctx).y + padding;
        ctx.draw(title);

        let description = Text::new(UNDEAD_FONT, &level.description)
            .position(Vector2::new(margin, y), Anchor::TopLeft)
            .scale(Vector2::repeat(state.config.ui_scale * 2.0))
            .max_width(WIDTH as f32 * tile_size - margin * 2.0)
            .z_index(layer::UI_ELEMENT);
        y -= description.size(ctx).y + padding;
        ctx.draw(description);

        let context = UIContext {
            scale,
            margin,
            tile_size,
            y: &mut y,
        };

        test_case(self, ctx, state, level, context);

        // Render backgrounds
        let height = ((ctx.size().y - y + margin) / tile_size).ceil() as usize;
        for yi in 0..height {
            for xi in 0..WIDTH {
                let mut pos =
                    Vector2::new(xi as f32 * tile_size, ctx.size().y - tile_size * yi as f32);

                if yi + 1 == height {
                    pos.y = y + tile_size - margin;
                }

                let side = (xi == WIDTH - 1) as i32 - (xi == 0) as i32;
                let uv_offset = Vector2::new(side * 16, 16 * (yi == height - 1) as i32);

                ctx.draw(
                    Sprite::new(INFO_PANEL)
                        .scale(Vector2::repeat(scale), Anchor::Center)
                        .position(pos, Anchor::TopLeft)
                        .uv_offset(uv_offset)
                        .z_index(layer::UI_BACKGROUND),
                );
            }
        }
    }
}

fn test_case(
    panel: &mut LevelPanel,
    ctx: &mut GraphicsContext<App>,
    state: &App,
    level: &Level,
    ui: UIContext,
) {
    let case = &level.tests.cases[panel.case];
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
            .scale(Vector2::repeat(scale), Anchor::Center)
            .z_index(layer::UI_ELEMENT)
    };

    let mut i = 0;
    for &input in &case.lasers {
        let pos = Vector2::new(ui.margin + i as f32 * tile_size, *ui.y);
        ctx.draw(
            case_tile(TILE_EMITTER_DOWN)
                .uv_offset(Vector2::new(-16 * input as i32, 0))
                .position(pos, Anchor::TopLeft),
        );
        i += 1;
    }

    ctx.draw(case_tile(BIG_RIGHT_ARROW).position(
        Vector2::new(ui.margin + i as f32 * tile_size, *ui.y - tile_size / 2.0),
        Anchor::CenterLeft,
    ));

    for &input in &case.detectors[0] {
        let pos = Vector2::new(ui.margin + i as f32 * tile_size + arrow_size, *ui.y);
        ctx.draw(
            case_tile(TILE_DETECTOR)
                .uv_offset(Vector2::new(16 * input as i32, 0))
                .position(pos, Anchor::TopLeft),
        );
        i += 1;
    }

    let case_idx = panel.case;
    let mut pos = Vector2::new(
        ui.tile_size * WIDTH as f32 - ui.margin,
        *ui.y - tile_size / 2.0,
    );
    let button_width = 4.0 * 3.0 * ctx.scale_factor * state.config.ui_scale;
    let button_padding = 4.0 * 2.0 * ctx.scale_factor * state.config.ui_scale;

    let mut case_button =
        |ctx: &mut GraphicsContext<App>, dir: bool, pos: Vector2<f32>| -> Sprite {
            let texture = if dir { RIGHT_ARROW } else { LEFT_ARROW };
            let mut case = case_tile(texture)
                .scale(Vector2::repeat(ui.scale), Anchor::Center)
                .position(pos, Anchor::CenterRight);

            if (!dir && panel.case == 0) || (dir && panel.case + 1 == level.tests.cases.len()) {
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

    *ui.y -= tile_size;
}
