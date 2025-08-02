use std::time::Duration;

use engine::{
    drawable::spacer::Spacer,
    exports::{nalgebra::Vector2, winit::event::MouseButton},
    graphics_context::{Anchor, GraphicsContext},
    layout::{Layout, LayoutElement, LayoutMethods, column::ColumnLayout, row::RowLayout},
    memory_key,
};

use crate::{
    app::App,
    consts::{ERROR_COLOR, layer},
    ui::{
        components::{
            button::{ButtonEffects, ButtonExt},
            horizontal_rule::Rule,
            modal::{Modal, modal_buttons},
            slider::Slider,
        },
        misc::body,
    },
    util::human_duration,
};

use super::GameScreen;

pub struct PausedModal {}

impl GameScreen {
    pub(super) fn paused_modal(&mut self, state: &mut App, ctx: &mut GraphicsContext) {
        if let Some(_pause) = &mut self.paused {
            // Don't add to time played when game is paused
            let delta = Duration::from_secs_f32(ctx.delta_time);
            self.board.transient.open_timestamp += delta;

            let (margin, padding) = state.spacing(ctx);
            let modal = Modal::new(state.modal_size(ctx))
                .position(ctx.center(), Anchor::Center)
                .margin(margin)
                .layer(layer::UI_OVERLAY);

            let size = modal.inner_size();
            modal.draw(ctx, |ctx, root| {
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
                        RowLayout::new(16.0 * ctx.scale_factor),
                        |ctx, layout| {
                            layout.nest(ctx, ColumnLayout::new(padding), |ctx, layout| {
                                let playtime = self.board.meta.playtime
                                    + self.board.transient.open_timestamp.elapsed().as_secs();
                                let playtime = format!("Playtime: {}", human_duration(playtime));
                                body(&playtime).layout(ctx, layout);

                                Spacer::new_y(padding).layout(ctx, layout);
                                body("Simulation Speed").layout(ctx, layout);
                                Slider::new(memory_key!()).layout(ctx, layout);

                                Spacer::new_y(padding / 2.0).layout(ctx, layout);
                                let trash_button = body("Delete World")
                                    .color(ERROR_COLOR)
                                    .button(memory_key!())
                                    .effects(ButtonEffects::Color);
                                trash = trash_button.is_clicked(ctx);
                                trash_button.layout(ctx, layout);
                            });

                            Rule::vertical(layout.available().y - 6.0 * ctx.scale_factor - padding)
                                .layout(ctx, layout);

                            layout.nest(ctx, ColumnLayout::new(padding), |ctx, layout| {
                                body("Controls")
                                    .scale(Vector2::repeat(3.0))
                                    .layout(ctx, layout);

                                body("T - Runs test cases\nP - Starts simulation\nSpace - Steps through the simulation\nQ - Copy hovered tile\nE - Toggle hovered emitter\nR - Rotates current tile")
                                    .layout(ctx, layout);
                            });
                        },
                    );

                    let clicking = ctx.input.mouse_down(MouseButton::Left);
                    let (exit, resume) = modal_buttons(ctx, layout, size.x, ("Exit", "Resume"));

                    if clicking && resume {
                        self.paused = None;
                    }

                    if trash || (clicking && exit) {
                        self.board.transient.trash |= trash;
                        state.pop_screen();
                    }
                });
            });
        }
    }
}
