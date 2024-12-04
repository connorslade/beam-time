use std::f32::consts::PI;

use beam_logic::level::Level;
use thousands::Separable;

use crate::{
    app::App,
    assets::{HISTOGRAM_BAR, UNDEAD_FONT},
    consts::layer,
};
use engine::{
    color::{OkLab, Rgb},
    drawable::{sprite::Sprite, text::Text},
    exports::nalgebra::{Vector2, Vector3},
    graphics_context::{Anchor, GraphicsContext},
};
use leaderboard::api::results::Histogram;

use super::{UIContext, WIDTH};

pub fn level_info(
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
    ui.text_block(ctx, state, &description);
}

pub fn level_complete(
    ctx: &mut GraphicsContext<App>,
    state: &App,
    board: &Level,
    latency: u32,
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
            // sprite.points.iter_mut().for_each(|point| point.y += offset);
        }
    });

    let text = format!(
        "Nice work! Your solution costs ${} and has a total latency of {latency} ticks.",
        price.separate_with_commas(),
    );
    ui.text_block(ctx, state, &text);
    ui.y -= ui.padding;

    let Some(hist_data) = state.leaderboard.get_results(board.id) else {
        ui.text_block(ctx, state, "Failed to load global data.");
        return;
    };

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

        let data = [&hist_data.cost, &hist_data.latency][i];

        let hist_pos = Vector2::new(ui.tile_size * WIDTH as f32 / 2.0 * i as f32, ui.y - offset);
        let height = histogram(ctx, state, ui, hist_pos, ui.tile_size, data, 75.0);
        ui.y -= (offset + height) * (i == 1) as u8 as f32;

        ctx.draw(text);
    }
}

pub fn level_failed(ctx: &mut GraphicsContext<App>, state: &App, case: usize, ui: &mut UIContext) {
    let center_x = (WIDTH as f32 * ui.tile_size) / 2.0;
    let title = Text::new(UNDEAD_FONT, "Level Failed...")
        .position(Vector2::new(center_x, ui.y), Anchor::TopCenter)
        .scale(Vector2::repeat(state.config.ui_scale * 3.0))
        .color(Rgb::hex(0xe43636))
        .z_index(layer::UI_ELEMENT);
    ui.y -= title.size(ctx).y + ui.scale + ui.padding;
    ctx.draw(title);

    let text = format!("Looks like you failed test case {case}. Check the board to see what went wrong then press ESC to exit the current simulation, make your fixes and re-run the tests.");
    ui.text_block(ctx, state, &text);
}

fn histogram(
    ctx: &mut GraphicsContext<App>,
    state: &App,
    ui: &mut UIContext,
    base: Vector2<f32>,
    height: f32,

    data: &Histogram,
    actual: f32,
) -> f32 {
    const BIN_COUNT: usize = 12;

    let max_count = data.bins.iter().max().copied().unwrap_or_default();
    let bars = data
        .bins
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
        Text::new(UNDEAD_FONT, data.max.to_string().as_str())
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

    let t = actual / data.max as f32;
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
