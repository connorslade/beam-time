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
            sprite.points.iter_mut().for_each(|point| point.y += offset);
        }
    });

    let text = format!(
        "Nice work! Your solution costs ${} and has a total latency of {latency} ticks.",
        price.separate_with_commas(),
    );
    ui.text_block(ctx, state, &text);
    ui.y -= ui.padding;

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

    let mut bins = [0; BIN_COUNT];
    for point in data {
        let bin = (point as f32 / bin_width) as usize;
        bins[bin.min(BIN_COUNT - 1)] += 1;
    }

    let max_count = *bins.iter().max().unwrap();

    dbg!(bins, max);

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
