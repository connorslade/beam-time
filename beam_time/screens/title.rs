use std::time::Instant;

use engine::{
    drawable::{spacer::Spacer, sprite::Sprite, text::Text},
    exports::{nalgebra::Vector2, winit::keyboard::KeyCode},
    graphics_context::{Anchor, Drawable, GraphicsContext},
    layout::{
        Justify, LayoutElement, LayoutMethods, column::ColumnLayout, root::RootLayout,
        row::RowLayout,
    },
    memory_key,
};

use crate::{
    App,
    assets::{ABOUT_BUTTON, CAMPAIGN_BUTTON, COPYRIGHT, SANDBOX_BUTTON, TITLE, UNDEAD_FONT},
    consts::{BACKGROUND_COLOR, layer},
    ui::{
        components::{button::Button, modal::Modal, slider::Slider},
        misc::body,
        waterfall::Waterfall,
    },
};

use super::{Screen, about::AboutScreen, campaign::CampaignScreen, sandbox::SandboxScreen};

pub struct TitleScreen {
    start_time: Instant,

    settings: Option<SettingsModal>,
}

pub struct SettingsModal {}

impl Screen for TitleScreen {
    fn render(&mut self, state: &mut App, ctx: &mut GraphicsContext) {
        ctx.background(BACKGROUND_COLOR);
        Waterfall::new(&mut state.waterfall).draw(ctx);
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

        #[allow(clippy::type_complexity)]
        let buttons: [(_, fn() -> Box<dyn Screen>); 3] = [
            (CAMPAIGN_BUTTON, || Box::new(CampaignScreen::default())),
            (SANDBOX_BUTTON, || Box::new(SandboxScreen::default())),
            (ABOUT_BUTTON, || Box::new(AboutScreen::default())),
        ];

        root.nest(
            ctx,
            ColumnLayout::new(padding).justify(Justify::Center),
            |ctx, layout| {
                Spacer::new_y(60.0 * ctx.scale_factor).layout(ctx, layout);
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
            let (margin, padding) = state.spacing(ctx);
            let modal = Modal::new(Vector2::new(ctx.center().x, 500.0))
                .position(ctx.center(), Anchor::Center)
                .margin(margin)
                .layer(layer::OVERLAY);

            let size = modal.inner_size();
            modal.draw(ctx, |ctx, root| {
                let body = body(size.x);

                root.nest(ctx, ColumnLayout::new(padding), |ctx, layout| {
                    body("Settings")
                        .scale(Vector2::repeat(4.0))
                        .layout(ctx, layout);
                    Spacer::new_y(4.0 * ctx.scale_factor).layout(ctx, layout);

                    let mut slider_setting = |name, value: &mut f32, range: (f32, f32)| {
                        body(name).layout(ctx, layout);
                        layout.nest(
                            ctx,
                            RowLayout::new(padding).justify(Justify::Center),
                            |ctx, layout| {
                                let slider = Slider::new(memory_key!(name))
                                    .default(*value)
                                    .bounds(range.0, range.1);
                                *value = slider.value(ctx);
                                slider.layout(ctx, layout);

                                Text::new(UNDEAD_FONT, format!("{value:.2}"))
                                    .scale(Vector2::repeat(2.0))
                                    .layout(ctx, layout);
                            },
                        );
                    };

                    slider_setting(
                        "Zoom Sensitivity",
                        &mut state.config.zoom_sensitivity,
                        (0.0, 2.0),
                    );

                    slider_setting(
                        "Movement Speed",
                        &mut state.config.movement_speed,
                        (1000.0, 3000.0),
                    );
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
