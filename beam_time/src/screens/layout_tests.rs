use engine::{
    color::Rgb,
    drawable::{
        shape::rectangle_outline::RectangleOutline, spacer::Spacer, sprite::Sprite, text::Text,
    },
    exports::nalgebra::Vector2,
    graphics_context::{Anchor, Drawable, GraphicsContext},
    layout::{
        column::ColumnLayout, root::RootLayout, row::RowLayout, tracker::LayoutTracker, Direction,
        Justify, Layout, LayoutElement, LayoutMethods,
    },
    memory_key,
};

use crate::{
    app::App,
    assets::{
        CAMPAIGN_BUTTON, OPTIONS_BUTTON, SANDBOX_BUTTON, TILE_EMITTER_RIGHT, TILE_MIRROR_A,
        UNDEAD_FONT,
    },
    consts::{ACCENT_COLOR, EMITTER, GALVO, MIRROR},
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

            root.nest(ctx, RowLayout::new(padding), |ctx, layout| {
                for tiles in [EMITTER, GALVO] {
                    layout.nest(ctx, ColumnLayout::new(padding), |ctx, layout| {
                        for tile in tiles {
                            Sprite::new(tile)
                                .uv_offset(Vector2::x() * 16)
                                .scale(Vector2::repeat(4.0))
                                .layout(ctx, layout);
                        }
                    });
                }
            });

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
                .color(Rgb::hex(0xFF0000))
                .draw(ctx);
        }

        {
            let mut root = RootLayout::new(ctx.center(), Anchor::Center);
            let mut column = ColumnLayout::new(padding).justify(Justify::Center);

            for (i, button) in [CAMPAIGN_BUTTON, SANDBOX_BUTTON, OPTIONS_BUTTON]
                .into_iter()
                .enumerate()
            {
                let key = memory_key!(i);
                let tracker = LayoutTracker::new(key);
                let hovered = tracker.hovered(ctx);

                let dt = ctx.delta_time * ((hovered as u8 as f32) * 2.0 - 1.0);
                let t = ctx.memory.get_or_insert(key, 0.0);
                *t = (*t + dt).clamp(0.0, 0.1);
                let color = Rgb::hex(0xFFFFFF).lerp(ACCENT_COLOR, *t / 0.1);

                Sprite::new(button)
                    .scale(Vector2::repeat(4.0))
                    .dynamic_scale(Vector2::repeat(4.0 + *t), Anchor::Center)
                    .color(color)
                    .tracked(tracker)
                    .layout(ctx, &mut column);
            }

            column.layout(ctx, &mut root);
            root.draw(ctx);
        }

        {
            let frame = ctx.memory.get_or_insert(memory_key!(), 0);
            *frame += 1;

            Text::new(UNDEAD_FONT, format!("Frame {frame}"))
                .position(
                    Vector2::new(padding, ctx.size().y - padding),
                    Anchor::TopLeft,
                )
                .scale(Vector2::repeat(2.0))
                .draw(ctx);
        }

        {
            let text = Text::new(UNDEAD_FONT, "Hello, World!")
                .position(ctx.center() + ctx.size() * 0.25, Anchor::BottomLeft)
                .scale(Vector2::repeat(2.0))
                .dynamic_scale(Vector2::repeat(2.0 + t.sin()), Anchor::Center);
            text.bounds(ctx)
                .outline()
                .color(Rgb::hex(0xFF0000))
                .draw(ctx);
            text.draw(ctx);
        }

        {
            let mut root =
                RootLayout::new(ctx.size() / 4.0, Anchor::Center).sized(Vector2::repeat(500.0));

            root.nest(ctx, RowLayout::new(padding), |ctx, layout| {
                for tile in MIRROR {
                    Sprite::new(tile)
                        .scale(Vector2::repeat(4.0))
                        .layout(ctx, layout);
                }

                layout.nest(
                    ctx,
                    RowLayout::new(padding)
                        .direction(Direction::MaxToMin)
                        .justify(Justify::Center),
                    |ctx, layout| {
                        Sprite::new(TILE_MIRROR_A)
                            .scale(Vector2::repeat(4.0))
                            .layout(ctx, layout);
                        Spacer::new(Vector2::x() * layout.available().x).layout(ctx, layout);
                    },
                );
            });

            root.draw(ctx);
        }
    }
}
