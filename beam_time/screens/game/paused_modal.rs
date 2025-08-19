use std::time::Duration;

use engine::{
    color::Rgb,
    drawable::{Anchor, spacer::Spacer, sprite::Sprite},
    exports::{nalgebra::Vector2, winit::event::MouseButton},
    graphics_context::GraphicsContext,
    layout::{
        Justify, Layout, LayoutElement, LayoutMethods, column::ColumnLayout, row::RowLayout,
        tracker::LayoutTracker,
    },
    memory_key,
};

use crate::{
    app::App,
    assets::{EDIT, RESET, TRASH},
    consts::{
        KEYBINDS, color, layer,
        spacing::{MARGIN, PADDING},
    },
    screens::game::ActiveModal,
    ui::{
        components::{
            horizontal_rule::Rule,
            key::Key,
            manual_button::ManualButton,
            modal::{Modal, modal_buttons},
            slider::Slider,
        },
        misc::{body, modal_size},
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

        let modal = Modal::new(modal_size(ctx))
            .position(ctx.center(), Anchor::Center)
            .margin(MARGIN)
            .layer(layer::UI_OVERLAY);
        modal.draw(ctx, |ctx, root| {
            let size = root.available();
            let body = body(size.x);

            let mut trash = false;
            root.nest(ctx, ColumnLayout::new(PADDING), |ctx, layout| {
                let name = match self.board.transient.level {
                    Some(level) => format!("Campaign: {}", level.name),
                    None => format!("Sandbox: {}", self.board.meta.name),
                };

                body(&name).scale(Vector2::repeat(4.0)).layout(ctx, layout);

                layout.nest(ctx, RowLayout::new(24.0), |ctx, layout| {
                    layout.nest(ctx, ColumnLayout::new(PADDING), |ctx, layout| {
                        Spacer::new_y(-PADDING / 2.0).layout(ctx, layout);

                        let playtime = self.board.total_playtime();
                        let playtime = format!("Playtime: {}", human_duration(playtime));
                        body(&playtime).layout(ctx, layout);
                        Spacer::new_y(PADDING / 2.0).layout(ctx, layout);

                        let mut icon_button = |icon, text| {
                            let key = memory_key!(icon);
                            let tracker = LayoutTracker::new(key);
                            let button = ManualButton::new(key).tracker(ctx, tracker);
                            let color =
                                Rgb::repeat(1.0).lerp(color::ACCENT, button.hover_time(ctx));

                            RowLayout::new(PADDING)
                                .justify(Justify::Center)
                                .tracked(tracker)
                                .show(ctx, layout, |ctx, layout| {
                                    Sprite::new(icon)
                                        .scale(Vector2::repeat(2.0))
                                        .color(color)
                                        .layout(ctx, layout);
                                    body(text).color(color).layout(ctx, layout);
                                });

                            button.pressed(ctx)
                        };

                        if self.board.transient.level.is_some() {
                            let solutions_text =
                                format!("Solutions ({})", self.solutions.len() + 1);
                            icon_button(EDIT, &solutions_text)
                                .then(|| self.modal = ActiveModal::Solutions);
                            icon_button(RESET, "Reset").then(|| self.modal = ActiveModal::Reset);
                        } else {
                            icon_button(TRASH, "Delete World").then(|| trash = true);
                        }

                        Spacer::new_y(PADDING / 2.0).layout(ctx, layout);
                        body("Simulation Speed (TPS)").layout(ctx, layout);
                        layout.nest(
                            ctx,
                            RowLayout::new(PADDING).justify(Justify::Center),
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

                    Rule::vertical(layout.available().y - 6.0 - PADDING * 2.0).layout(ctx, layout);

                    layout.nest(ctx, ColumnLayout::new(PADDING / 2.0), |ctx, layout| {
                        Spacer::new_y(0.0).layout(ctx, layout);

                        body("Use WASD or middle mouse + drag to pan and scroll to zoom.")
                            .layout(ctx, layout);
                        Spacer::new_y(PADDING / 2.0).layout(ctx, layout);

                        let skip = self.board.transient.level.is_none() as usize;
                        for (key, desc) in KEYBINDS.iter().skip(skip) {
                            layout.nest(
                                ctx,
                                RowLayout::new(PADDING / 2.0).justify(Justify::Center),
                                |ctx, layout| {
                                    Key::new(key).scale(2.0).layout(ctx, layout);
                                    body("-").layout(ctx, layout);
                                    body(desc).layout(ctx, layout);
                                },
                            );
                        }
                    });
                });

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
