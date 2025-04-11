use std::time::Instant;

use engine::{
    color::Rgb,
    drawable::{spacer::Spacer, sprite::Sprite, text::Text},
    exports::{nalgebra::Vector2, winit::keyboard::KeyCode},
    graphics_context::{Anchor, Drawable, GraphicsContext},
    layout::{column::ColumnLayout, root::RootLayout, Justify, LayoutElement, LayoutMethods},
    memory_key,
};

use crate::{
    assets::{ABOUT_BUTTON, CAMPAIGN_BUTTON, COPYRIGHT, SANDBOX_BUTTON, TITLE, UNDEAD_FONT},
    consts::{layer, BACKGROUND_COLOR},
    ui::{
        components::{button::Button, modal::Modal},
        waterfall::Waterfall,
    },
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
        self.setting_modal(state, ctx);

        // Replace with a settings button or smth
        if ctx.input.consume_key_pressed(KeyCode::KeyS) {
            self.settings = Some(SettingsModal {});
        }

        if self.settings.is_some() && ctx.input.consume_key_pressed(KeyCode::Escape) {
            self.settings = None;
        }

        // Title & copyright
        let pos = Vector2::new(ctx.size().x / 2.0, ctx.size().y * 0.9);
        let t = self.start_time.elapsed().as_secs_f32().sin() / 20.0;

        Sprite::new(TITLE)
            .position(pos, Anchor::TopCenter)
            .scale(Vector2::repeat(6.0))
            .rotate(t, Anchor::Center)
            .draw(ctx);

        Sprite::new(COPYRIGHT)
            .position(Vector2::new(ctx.size().x - 10.0, 10.0), Anchor::BottomRight)
            .scale(Vector2::repeat(2.0))
            .draw(ctx);

        let mut root = RootLayout::new(ctx.center(), Anchor::Center);
        let (_, padding) = state.spacing(ctx);

        let buttons: [(_, fn() -> Box<dyn Screen>); 3] = [
            (CAMPAIGN_BUTTON, || Box::new(CampaignScreen::default())),
            (SANDBOX_BUTTON, || Box::new(SandboxScreen::default())),
            (ABOUT_BUTTON, || Box::new(AboutScreen::default())),
        ];

        root.nest(
            ctx,
            ColumnLayout::new(padding).justify(Justify::Center),
            |ctx, layout| {
                Spacer::new_y(64.0 * ctx.scale_factor).layout(ctx, layout);
                for (sprite, on_click) in buttons {
                    let key = memory_key!(sprite);
                    let button = Button::new(sprite, key).scale(Vector2::repeat(4.0));

                    button
                        .is_clicked(ctx)
                        .then(|| state.push_boxed_screen(on_click()));
                    button.layout(ctx, layout);
                }
            },
        );

        root.draw(ctx);
    }

    fn on_resize(&mut self, state: &mut App, _old_size: Vector2<f32>, _size: Vector2<f32>) {
        state.waterfall.reset();
    }
}

impl TitleScreen {
    fn setting_modal(&mut self, state: &mut App, ctx: &mut GraphicsContext) {
        if let Some(ref _settings) = self.settings {
            ctx.defer(|ctx| ctx.darken(Rgb::repeat(0.5), layer::OVERLAY));

            let (margin, padding) = state.spacing(ctx);
            let modal = Modal::new(Vector2::new(ctx.center().x, 500.0))
                .margin(margin)
                .layer(layer::OVERLAY);

            modal.draw(ctx, |ctx, root| {
                let body = |text| Text::new(UNDEAD_FONT, text).scale(Vector2::repeat(2.0));

                root.nest(ctx, ColumnLayout::new(padding), |ctx, layout| {
                    body("Settings")
                        .scale(Vector2::repeat(4.0))
                        .layout(ctx, layout);
                });
            });
        }
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
