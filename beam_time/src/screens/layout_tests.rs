use engine::{
    color::Rgb,
    drawable::{shape::rectangle_outline::RectangleOutline, sprite::Sprite},
    exports::nalgebra::Vector2,
    graphics_context::{Anchor, Drawable, GraphicsContext},
    layout::{column::ColumnLayout, root::RootLayout, row::RowLayout, Justify},
};

use crate::{
    app::App,
    assets::{CAMPAIGN_BUTTON, OPTIONS_BUTTON, SANDBOX_BUTTON, TILE_EMITTER_RIGHT},
    consts::{EMITTER, GALVO},
};

use super::Screen;

#[derive(Default)]
pub struct LayoutTestScreen {}

impl Screen for LayoutTestScreen {
    fn render(&mut self, state: &mut App, ctx: &mut GraphicsContext) {
        let t = state.start.elapsed().as_secs_f32();
        let pos = ctx.input.mouse;

        let padding = (16.0 + 16.0 * ((2.0 * t).sin() / 2.0 - 0.5))
            * state.config.ui_scale
            * ctx.scale_factor;

        {
            let mut root = RootLayout::new(pos, Anchor::TopLeft);
            let mut row = RowLayout::new(padding);

            for tiles in [EMITTER, GALVO] {
                let mut column = ColumnLayout::new(padding);

                for tile in tiles {
                    column.layout(
                        ctx,
                        Sprite::new(tile)
                            .uv_offset(Vector2::x() * 16)
                            .scale(Vector2::repeat(4.0)),
                    );
                }

                row.layout(ctx, column);
            }

            root.layout(ctx, row);
            root.draw(ctx);
        }

        {
            let pos = ctx.center() + Vector2::new(t.cos(), t.sin()) * 350.0;
            let scale = Vector2::repeat(4.0 + (t.sin() / 2.0) - 0.5);
            Sprite::new(TILE_EMITTER_RIGHT)
                .uv_offset(Vector2::x() * 16)
                .scale(Vector2::repeat(4.0))
                .dynamic_scale(scale, Anchor::Center)
                .position(pos, Anchor::Center)
                .draw(ctx);
            RectangleOutline::new(Vector2::repeat(4.0) * 16.0 * ctx.scale_factor, 1.0)
                .position(pos, Anchor::Center)
                .color(Rgb::hex(0x69bdd2))
                .draw(ctx);
        }

        {
            let mut root = RootLayout::new(ctx.center(), Anchor::Center);
            let mut column = ColumnLayout::new(padding).justify(Justify::Center);

            for button in [CAMPAIGN_BUTTON, SANDBOX_BUTTON, OPTIONS_BUTTON] {
                column.layout(ctx, Sprite::new(button).scale(Vector2::repeat(4.0)));
            }

            root.layout(ctx, column);
            root.draw(ctx);
        }
    }
}
