use std::time::Instant;

use engine::{
    color::Rgb,
    drawable::{sprite::Sprite, text::Text},
    exports::{nalgebra::Vector2, winit::keyboard::KeyCode},
    graphics_context::{Anchor, GraphicsContext},
    memory_key,
};

use crate::{
    assets::{ABOUT_BUTTON, CAMPAIGN_BUTTON, COPYRIGHT, SANDBOX_BUTTON, TITLE, UNDEAD_FONT},
    consts::{layer, BACKGROUND_COLOR},
    ui::{button::Button, layout::column::ColumnLayout, modal::Modal, waterfall::Waterfall},
    App,
};

use super::{about::AboutScreen, campaign::CampaignScreen, sandbox::SandboxScreen, Screen};

pub struct TitleScreen {
    start_time: Instant,

    settings: Option<SettingsModal>,
}

pub struct SettingsModal {}

impl Screen for TitleScreen {
    fn render(&mut self, state: &mut App, ctx: &mut GraphicsContext) {
        ctx.background(BACKGROUND_COLOR);
        ctx.draw(Waterfall::new(&mut state.waterfall));

        // Replace with a settings button or smth
        if ctx.input.consume_key_pressed(KeyCode::KeyS) {
            self.settings = Some(SettingsModal {});
        }

        if self.settings.is_some() && ctx.input.consume_key_pressed(KeyCode::Escape) {
            self.settings = None;
        }

        if let Some(ref _settings) = self.settings {
            ctx.defer(|ctx| ctx.darken(Rgb::repeat(0.5), layer::OVERLAY));

            let (margin, padding) = state.spacing(ctx);
            let modal = Modal::new(Vector2::new(ctx.center().x, 500.0))
                .margin(margin)
                .layer(layer::OVERLAY);

            modal.draw(ctx, |ctx| {
                let mut layout = ColumnLayout::new(padding);
                let body = |text| Text::new(UNDEAD_FONT, text).scale(Vector2::repeat(2.0));
                layout.draw(ctx, body("Settings").scale(Vector2::repeat(4.0)));
            });
        }

        // Title & copyright
        let pos = Vector2::new(ctx.size().x / 2.0, ctx.size().y * 0.9);
        let t = self.start_time.elapsed().as_secs_f32().sin() / 20.0;
        ctx.draw(
            Sprite::new(TITLE)
                .position(pos, Anchor::TopCenter)
                .scale(Vector2::repeat(6.0))
                .rotate(t, Anchor::Center),
        );

        ctx.draw(
            Sprite::new(COPYRIGHT)
                .position(Vector2::new(ctx.size().x - 10.0, 10.0), Anchor::BottomRight)
                .scale(Vector2::repeat(2.0)),
        );

        // Buttons
        let campaign_button = Button::new(CAMPAIGN_BUTTON, memory_key!())
            .pos(ctx.center(), Anchor::Center)
            .scale(Vector2::repeat(4.0));
        if campaign_button.is_clicked(ctx) {
            state.push_screen(CampaignScreen::default())
        }

        let sandbox_button = Button::new(SANDBOX_BUTTON, memory_key!())
            .pos(
                ctx.center() - Vector2::new(0.0, 14.0 * 5.0 * ctx.scale_factor),
                Anchor::Center,
            )
            .scale(Vector2::repeat(4.0));
        if sandbox_button.is_clicked(ctx) {
            state.push_screen(SandboxScreen::default());
        }

        let about_button = Button::new(ABOUT_BUTTON, memory_key!())
            .pos(
                ctx.center() - Vector2::new(0.0, 2.0 * 14.0 * 5.0 * ctx.scale_factor),
                Anchor::Center,
            )
            .scale(Vector2::repeat(4.0));
        if about_button.is_clicked(ctx) {
            state.push_screen(AboutScreen::default())
        }

        ctx.draw([campaign_button, sandbox_button, about_button]);
    }

    fn on_resize(&mut self, state: &mut App, _old_size: Vector2<f32>, _size: Vector2<f32>) {
        state.waterfall.reset();
    }
}

impl Default for TitleScreen {
    fn default() -> Self {
        Self {
            start_time: Instant::now(),

            settings: None,
        }
    }
}
