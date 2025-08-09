use std::path::PathBuf;

use engine::{
    drawable::{Anchor, dummy::DummyDrawable, spacer::Spacer, sprite::Sprite, text::Text},
    exports::{nalgebra::Vector2, winit::event::MouseButton},
    graphics_context::GraphicsContext,
    layout::{
        Direction, Justify, Layout, LayoutElement, LayoutMethods, column::ColumnLayout,
        row::RowLayout,
    },
    memory_key,
};
use thousands::Separable;

use crate::{
    app::App,
    assets::{DUPLICATE, EDIT, TRASH, UNDEAD_FONT},
    consts::layer,
    game::board::{BoardMeta, LevelStats, unloaded::UnloadedBoard},
    screens::game::{ActiveModal, GameScreen},
    ui::{
        components::{
            button::{ButtonEffects, ButtonExt},
            horizontal_rule::Rule,
            modal::{Modal, modal_buttons},
        },
        misc::{body, modal_size, spacing},
    },
};

impl GameScreen {
    pub(super) fn solutions_modal(&mut self, _state: &mut App, ctx: &mut GraphicsContext) {
        let (Some(level), ActiveModal::Solutions) = (self.board.transient.level, &self.modal)
        else {
            return;
        };

        let (margin, padding) = spacing(ctx);
        let modal = Modal::new(modal_size(ctx))
            .position(ctx.center(), Anchor::Center)
            .margin(margin)
            .layer(layer::UI_OVERLAY);
        modal.draw(ctx, |ctx, root| {
            let size = root.available();
            let body = body(size.x);

            ColumnLayout::new(padding)
                .justify(Justify::Center)
                .show(ctx, root, |ctx, layout| {
                    RowLayout::new(0.0).show(ctx, layout, |ctx, layout| {
                        body(&format!("{} Solutions", level.name))
                            .scale(Vector2::repeat(4.0))
                            .layout(ctx, layout);
                        Spacer::new_x(layout.available().x).layout(ctx, layout);
                    });
                    DummyDrawable::new().layout(ctx, layout);

                    Rule::horizontal(layout.available().x).layout(ctx, layout);
                    solution(ctx, layout, &self.save_file, &self.board.meta);
                    Rule::horizontal(layout.available().x).layout(ctx, layout);
                    for UnloadedBoard { path, meta } in &self.solutions {
                        solution(ctx, layout, path, meta);
                        Rule::horizontal(layout.available().x).layout(ctx, layout);
                    }

                    Text::new(UNDEAD_FONT, "+ New Solution +")
                        .scale(Vector2::repeat(2.0))
                        .button(memory_key!())
                        .layout(ctx, layout);

                    let (back, _) = modal_buttons(ctx, layout, size.x, ("Back", ""));
                    let click = ctx.input.mouse_pressed(MouseButton::Left);
                    if click && back {
                        self.modal = ActiveModal::Paused;
                        ctx.input.cancel_clicks()
                    }
                });
        });
    }
}

fn solution(
    ctx: &mut GraphicsContext,
    layout: &mut ColumnLayout,
    path: &PathBuf,
    meta: &BoardMeta,
) {
    let (_, padding) = spacing(ctx);
    ColumnLayout::new(padding).show(ctx, layout, |ctx, layout| {
        RowLayout::new(0.0)
            .justify(Justify::Center)
            .sized(Vector2::new(layout.available().x, 0.0))
            .show(ctx, layout, |ctx, layout| {
                Text::new(UNDEAD_FONT, &format!("{} (Current)", meta.name))
                    .scale(Vector2::repeat(3.0))
                    .button(memory_key!(path))
                    .effects(ButtonEffects::empty())
                    .layout(ctx, layout);

                let row = RowLayout::new(padding).direction(Direction::MaxToMin);
                row.show(ctx, layout, |ctx, layout| {
                    let button = |asset| {
                        Sprite::new(asset)
                            .scale(Vector2::repeat(2.0))
                            .button(memory_key!(path, asset))
                    };

                    button(TRASH).layout(ctx, layout);
                    button(DUPLICATE).layout(ctx, layout);
                    button(EDIT).layout(ctx, layout);

                    Spacer::new_x(layout.available().x).layout(ctx, layout);
                });
            });

        let level = meta.level.as_ref().unwrap();
        let text = if let Some(LevelStats { cost, latency }) = level.solved {
            let cost = cost.separate_with_commas();
            format!("Costs ${cost} • Latency of {latency} ticks")
        } else {
            "Unsolved".into()
        };

        Text::new(UNDEAD_FONT, &text)
            .scale(Vector2::repeat(2.0))
            .layout(ctx, layout);
    });
}
