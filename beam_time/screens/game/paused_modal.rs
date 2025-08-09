use std::time::Duration;

use engine::{
    drawable::{Anchor, spacer::Spacer, sprite::Sprite},
    exports::{nalgebra::Vector2, winit::event::MouseButton},
    graphics_context::GraphicsContext,
    layout::{Justify, Layout, LayoutElement, LayoutMethods, column::ColumnLayout, row::RowLayout},
    memory_key,
};

use crate::{
    app::App,
    assets::{EDIT, RESET, TRASH},
    consts::{KEYBINDS, color, layer},
    screens::game::ActiveModal,
    ui::{
        components::{
            button::{ButtonEffects, ButtonExt},
            horizontal_rule::Rule,
            key::Key,
            modal::{Modal, modal_buttons},
            slider::Slider,
        },
        misc::{body, modal_size, spacing},
    },
    util::time::human_duration,
};

use super::GameScreen;

impl GameScreen {
    pub(super) fn paused_modal(&mut self, state: &mut App, ctx: &mut GraphicsContext) {
        if !matches!(self.modal, ActiveModal::Paused) {
            return;
        }

        // Don't add to time played when game is paused
        let delta = Duration::from_secs_f32(ctx.delta_time);
        self.board.transient.open_timestamp += delta;

        let (margin, padding) = spacing(ctx);
        let modal = Modal::new(modal_size(ctx))
            .position(ctx.center(), Anchor::Center)
            .margin(margin)
            .layer(layer::UI_OVERLAY);
        modal.draw(ctx, |ctx, root| {
            let size = root.available();
            let body = body(size.x);

            let mut trash = false;
            root.nest(ctx, ColumnLayout::new(padding), |ctx, layout| {
                let name = match self.board.transient.level {
                    Some(level) => format!("Campaign: {}", level.name),
                    None => format!("Sandbox: {}", self.board.meta.name),
                };

                body(&name).scale(Vector2::repeat(4.0)).layout(ctx, layout);

                layout.nest(
                    ctx,
                    RowLayout::new(24.0 * ctx.scale_factor),
                    |ctx, layout| {
                        layout.nest(ctx, ColumnLayout::new(padding), |ctx, layout| {
                            Spacer::new_y(-padding / 2.0).layout(ctx, layout);

                            let playtime = self.board.total_playtime();
                            let playtime = format!("Playtime: {}", human_duration(playtime));
                            body(&playtime).layout(ctx, layout);
                            Spacer::new_y(padding / 2.0).layout(ctx, layout);

                            if self.board.transient.level.is_some() {
                                layout.nest(
                                    ctx,
                                    RowLayout::new(padding).justify(Justify::Center),
                                    |ctx, layout| {
                                        Sprite::new(EDIT)
                                            .scale(Vector2::repeat(2.0))
                                            .layout(ctx, layout);
                                        body(&format!("Solutions ({})", self.solutions.len() + 1))
                                            .button(memory_key!())
                                            .effects(ButtonEffects::Color)
                                            .on_click(ctx, || self.modal = ActiveModal::Solutions)
                                            .layout(ctx, layout);
                                    },
                                );

                                layout.nest(
                                    ctx,
                                    RowLayout::new(padding).justify(Justify::Center),
                                    |ctx, layout| {
                                        Sprite::new(RESET)
                                            .scale(Vector2::repeat(2.0))
                                            .layout(ctx, layout);
                                        body("Reset")
                                            .button(memory_key!())
                                            .effects(ButtonEffects::Color)
                                            .on_click(ctx, || {
                                                self.beam.get().beam = None;
                                                self.board.reset();
                                                self.modal = ActiveModal::None;
                                            })
                                            .layout(ctx, layout);
                                    },
                                );
                            } else {
                                layout.nest(
                                    ctx,
                                    RowLayout::new(padding).justify(Justify::Center),
                                    |ctx, layout| {
                                        Sprite::new(TRASH)
                                            .scale(Vector2::repeat(2.0))
                                            .color(color::ERROR)
                                            .layout(ctx, layout);
                                        body("Delete World")
                                            .color(color::ERROR)
                                            .button(memory_key!())
                                            .effects(ButtonEffects::Color)
                                            .on_click(ctx, || trash = true)
                                            .layout(ctx, layout);
                                    },
                                );
                            }

                            Spacer::new_y(padding / 2.0).layout(ctx, layout);
                            body("Simulation Speed (TPS)").layout(ctx, layout);
                            layout.nest(
                                ctx,
                                RowLayout::new(padding).justify(Justify::Center),
                                |ctx, layout| {
                                    let slider = Slider::new(memory_key!())
                                        .bounds(1.0, 100.0)
                                        .default(20.0)
                                        .start(self.tps);
                                    self.tps = slider.value(ctx);
                                    slider.layout(ctx, layout);

                                    if self.tps == 100.0 {
                                        body("∞").layout(ctx, layout);
                                        self.tps = f32::MAX;
                                    } else {
                                        body(&format!("{:.0}", self.tps)).layout(ctx, layout);
                                    }
                                },
                            );
                        });

                        Rule::vertical(
                            layout.available().y - 6.0 * ctx.scale_factor - padding * 2.0,
                        )
                        .layout(ctx, layout);

                        layout.nest(ctx, ColumnLayout::new(padding / 2.0), |ctx, layout| {
                            Spacer::new_y(0.0).layout(ctx, layout);

                            body("Use WASD or middle mouse + drag to pan and scroll to zoom.")
                                .layout(ctx, layout);
                            Spacer::new_y(padding / 2.0).layout(ctx, layout);

                            let skip = self.board.transient.level.is_none() as usize;
                            for (key, desc) in KEYBINDS.iter().skip(skip) {
                                layout.nest(
                                    ctx,
                                    RowLayout::new(padding / 2.0).justify(Justify::Center),
                                    |ctx, layout| {
                                        Key::new(key).scale(2.0).layout(ctx, layout);
                                        body("-").layout(ctx, layout);
                                        body(desc).layout(ctx, layout);
                                    },
                                );
                            }
                        });
                    },
                );

                let clicking = ctx.input.mouse_pressed(MouseButton::Left);
                let (exit, resume) = modal_buttons(ctx, layout, size.x, ("Exit", "Resume"));

                (clicking && resume).then(|| self.modal = ActiveModal::None);
                if trash || (clicking && exit) {
                    self.board.transient.trash |= trash;
                    state.pop_screen();
                }
            });
        });
    }
}
