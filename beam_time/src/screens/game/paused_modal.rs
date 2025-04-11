use std::time::Duration;

use engine::{
    color::Rgb,
    drawable::spacer::Spacer,
    exports::{nalgebra::Vector2, winit::event::MouseButton},
    graphics_context::GraphicsContext,
    layout::{
        column::ColumnLayout, row::RowLayout, tracker::LayoutTracker, Direction, Layout,
        LayoutElement, LayoutMethods,
    },
    memory_key,
};

use crate::{
    app::App,
    assets::TRASH,
    consts::layer,
    ui::{
        components::{
            button::Button,
            modal::{modal_buttons, Modal},
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

            ctx.defer(|ctx| ctx.darken(Rgb::repeat(0.5), layer::UI_OVERLAY));

            let (margin, padding) = state.spacing(ctx);
            let modal = Modal::new(Vector2::new(ctx.center().x, 500.0))
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

                    layout.nest(ctx, RowLayout::new(padding), |ctx, layout| {
                        body(&name).scale(Vector2::repeat(4.0)).layout(ctx, layout);

                        layout.nest(
                            ctx,
                            RowLayout::new(padding).direction(Direction::MaxToMin),
                            |ctx, layout| {
                                let tracker = LayoutTracker::new(memory_key!());
                                trash = tracker.clicked(ctx, MouseButton::Left);
                                Button::new(TRASH, memory_key!())
                                    .scale(Vector2::repeat(2.0))
                                    .tracked(tracker)
                                    .layout(ctx, layout);
                                Spacer::new_x(layout.available().x).layout(ctx, layout);
                            },
                        );
                    });

                    let playtime = self.board.meta.playtime
                        + self.board.transient.open_timestamp.elapsed().as_secs();
                    let playtime = format!("Playtime: {}", human_duration(playtime));
                    body(&playtime).layout(ctx, layout);

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
