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

use crate::{
    app::App,
    assets::{DUPLICATE, EDIT, TRASH, UNDEAD_FONT},
    consts::layer,
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

                    let i = 0;

                    ColumnLayout::new(padding).show(ctx, layout, |ctx, layout| {
                        RowLayout::new(0.0)
                            .justify(Justify::Center)
                            .sized(Vector2::new(size.x, 0.0))
                            .show(ctx, layout, |ctx, layout| {
                                Text::new(UNDEAD_FONT, "Solution #1")
                                    .scale(Vector2::repeat(3.0))
                                    .button(memory_key!(i))
                                    .effects(ButtonEffects::empty())
                                    .layout(ctx, layout);

                                let row = RowLayout::new(padding).direction(Direction::MaxToMin);
                                row.show(ctx, layout, |ctx, layout| {
                                    let button = |asset| {
                                        Sprite::new(asset)
                                            .scale(Vector2::repeat(2.0))
                                            .button(memory_key!(i, asset))
                                    };

                                    button(TRASH).layout(ctx, layout);
                                    button(DUPLICATE).layout(ctx, layout);
                                    button(EDIT).layout(ctx, layout);

                                    Spacer::new_x(layout.available().x).layout(ctx, layout);
                                });
                            });

                        Text::new(UNDEAD_FONT, "Costs $23,000 • Latency of 147 ticks")
                            .scale(Vector2::repeat(2.0))
                            .layout(ctx, layout);
                    });

                    Rule::horizontal(layout.available().x).layout(ctx, layout);

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
