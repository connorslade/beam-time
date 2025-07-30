use std::{mem, time::Instant};

use engine::{
    drawable::{spacer::Spacer, sprite::Sprite, text::Text},
    exports::{
        nalgebra::Vector2,
        winit::{event::MouseButton, keyboard::KeyCode},
    },
    graphics_context::{Anchor, Drawable, GraphicsContext},
    layout::{
        Justify, LayoutElement, LayoutMethods, column::ColumnLayout, root::RootLayout,
        row::RowLayout,
    },
    memory_key,
};

use crate::{
    App,
    assets::{COPYRIGHT, TITLE, UNDEAD_FONT},
    consts::{BACKGROUND_COLOR, layer},
    ui::{
        components::{
            button::{ButtonEffects, ButtonExt},
            modal::{Modal, modal_buttons},
            slider::Slider,
        },
        misc::body,
        waterfall::Waterfall,
    },
};

use super::{Screen, about::AboutScreen, campaign::CampaignScreen, sandbox::SandboxScreen};

pub struct TitleScreen {
    start_time: Instant,

    settings: Option<SettingsModal>,
}

pub struct SettingsModal;

impl Screen for TitleScreen {
    fn render(&mut self, state: &mut App, ctx: &mut GraphicsContext) {
        ctx.background(BACKGROUND_COLOR);
        Waterfall::new(&mut state.waterfall).draw(ctx);
        self.setting_modal(state, ctx);

        if ctx.input.consume_key_pressed(KeyCode::Escape)
            && let Some(_settings) = mem::take(&mut self.settings)
        {}

        // Title & copyright
        let pos = Vector2::new(ctx.size().x / 2.0, ctx.size().y * 0.9);
        let t = self.start_time.elapsed().as_secs_f32().sin() / 20.0;

        let screen = ctx.size() / ctx.scale_factor;
        let size = ctx.assets.get_sprite(TITLE).size;
        let scale = (screen.x / size.x as f32 * 0.5)
            .min(screen.y / size.y as f32 * 0.3)
            .clamp(4.0, 15.0);

        Sprite::new(TITLE)
            .position(pos, Anchor::TopCenter)
            .scale(Vector2::repeat(scale))
            .rotate(t, Anchor::Center)
            .draw(ctx);

        Sprite::new(COPYRIGHT)
            .position(Vector2::new(ctx.size().x - 10.0, 10.0), Anchor::BottomRight)
            .scale(Vector2::repeat(2.0))
            .draw(ctx);

        let mut root = RootLayout::new(
            Vector2::new(ctx.center().x, ctx.size().y * 0.45),
            Anchor::Center,
        );
        let (_, padding) = state.spacing(ctx);

        let buttons: [(_, fn(&mut App, &mut _)); _] = [
            ("Campaign", |s, _| s.push_screen(CampaignScreen::default())),
            ("Sandbox", |s, _| s.push_screen(SandboxScreen::default())),
            ("Settings", |_, settings| *settings = Some(SettingsModal {})),
            ("About", |s, _| s.push_screen(AboutScreen::default())),
        ];

        root.nest(
            ctx,
            ColumnLayout::new(padding).justify(Justify::Center),
            |ctx, layout| {
                Spacer::new_y(60.0 * ctx.scale_factor).layout(ctx, layout);
                for (name, on_click) in buttons {
                    let key = memory_key!(name);
                    let button = Text::new(UNDEAD_FONT, name)
                        .scale(Vector2::repeat(4.0))
                        .button(key)
                        .effects(ButtonEffects::Color | ButtonEffects::Arrows);

                    button
                        .is_clicked(ctx)
                        .then(|| on_click(state, &mut self.settings));
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
        if let Some(_settings) = &mut self.settings {
            let (margin, padding) = state.spacing(ctx);
            let modal = Modal::new(state.modal_size(ctx))
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
                        let mut out = false;
                        layout.nest(
                            ctx,
                            RowLayout::new(padding).justify(Justify::Center),
                            |ctx, layout| {
                                let slider = Slider::new(memory_key!(name))
                                    .default(*value)
                                    .bounds(range.0, range.1);
                                *value = slider.value(ctx);
                                out = slider.is_dragging(ctx);
                                slider.layout(ctx, layout);

                                Text::new(UNDEAD_FONT, format!("{value:.2}"))
                                    .scale(Vector2::repeat(2.0))
                                    .layout(ctx, layout);
                            },
                        );
                        out
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

                    let clicking = ctx.input.mouse_down(MouseButton::Left);
                    let (back, _) = modal_buttons(ctx, layout, size.x, ("Back", ""));
                    (back && clicking).then(|| self.settings = None);
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
