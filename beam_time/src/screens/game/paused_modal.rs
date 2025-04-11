use std::time::Duration;

use engine::{
    color::Rgb,
    drawable::text::Text,
    exports::{nalgebra::Vector2, winit::event::MouseButton},
    graphics_context::GraphicsContext,
    layout::{column::ColumnLayout, LayoutElement},
};

use crate::{
    app::App,
    assets::UNDEAD_FONT,
    consts::layer,
    ui::{components::modal::Modal, misc::modal_buttons_old},
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

            let origin = modal.origin(ctx);
            let size = modal.inner_size();
            modal.draw(ctx, |ctx, root| {
                let body = |text| {
                    Text::new(UNDEAD_FONT, text)
                        .scale(Vector2::repeat(2.0))
                        .max_width(size.x)
                };

                let mut column = ColumnLayout::new(padding);
                let name = match self.board.transient.level {
                    Some(level) => format!("Campaign: {}", level.name),
                    None => format!("Sandbox: {}", self.board.meta.name),
                };

                body(&name)
                    .scale(Vector2::repeat(4.0))
                    .layout(ctx, &mut column);

                let playtime = self.board.meta.playtime
                    + self.board.transient.open_timestamp.elapsed().as_secs();
                let playtime = format!("Playtime: {}", human_duration(playtime));
                body(&playtime).layout(ctx, &mut column);

                column.layout(ctx, root);

                let clicking = ctx.input.mouse_down(MouseButton::Left);
                let (exit, resume) = modal_buttons_old(
                    ctx,
                    origin + Vector2::new(margin, -size.y - ctx.scale_factor * 12.0),
                    size.x,
                    ("Exit", "Resume"),
                );

                if clicking && resume {
                    self.paused = None;
                }

                if clicking && exit {
                    state.pop_screen();
                }
            });
        }
    }
}
