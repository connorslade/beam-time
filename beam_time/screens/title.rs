use std::mem;

use engine::{
    color::Rgb,
    drawable::{spacer::Spacer, text::Text},
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
    assets::{ALAGARD_FONT, UNDEAD_FONT},
    consts::{AUTHOR_HOMEPAGE, BACKGROUND_COLOR, DESCRIPTION, GAME_HOMEPAGE, layer},
    ui::{
        components::{
            button::{ButtonEffects, ButtonExt},
            checkbox::checkbox,
            modal::{Modal, modal_buttons},
            slider::Slider,
        },
        misc::body,
        waterfall::Waterfall,
    },
};

use super::{Screen, campaign::CampaignScreen, sandbox::SandboxScreen};

pub struct TitleScreen {
    modal: ActiveModal,
}

#[derive(Default)]
enum ActiveModal {
    #[default]
    None,
    Settings,
    About,
}

impl Screen for TitleScreen {
    fn render(&mut self, state: &mut App, ctx: &mut GraphicsContext) {
        let (margin, padding) = state.spacing(ctx);

        ctx.background(BACKGROUND_COLOR);
        Waterfall::new(&mut state.waterfall).draw(ctx);
        self.modals(state, ctx);

        ctx.input
            .consume_key_pressed(KeyCode::Escape)
            .then(|| mem::take(&mut self.modal));

        // Title & copyright
        let pos = Vector2::new(ctx.size().x / 2.0, ctx.size().y * 0.9);

        let screen = ctx.size() / ctx.scale_factor;
        let scale = (screen.x / 81.0 * 0.5)
            .min(screen.y / 20.0 * 0.3)
            .clamp(4.0, 15.0);

        let title = Text::new(ALAGARD_FONT, "Beam Time")
            .position(pos, Anchor::TopCenter)
            .scale(Vector2::repeat(scale.round()))
            .default_shadow();
        let size = title.size(ctx);
        let title_button = title
            .button(memory_key!())
            .effects(ButtonEffects::Arrows | ButtonEffects::Color);
        title_button
            .is_clicked(ctx)
            .then(|| webbrowser::open(GAME_HOMEPAGE));
        title_button.draw(ctx);

        let offset = Vector2::new(size.x / 2.0, -size.y * 1.25);
        let author_button = Text::new(UNDEAD_FONT, "By Connor Slade")
            .position(pos + offset, Anchor::TopRight)
            .scale(Vector2::repeat((scale / 2.0).round()))
            .default_shadow()
            .button(memory_key!())
            .effects(ButtonEffects::Arrows | ButtonEffects::Color);
        author_button
            .is_clicked(ctx)
            .then(|| webbrowser::open(AUTHOR_HOMEPAGE));
        author_button.draw(ctx);

        Text::new(
            UNDEAD_FONT,
            format!("v{} - ©2025", env!("CARGO_PKG_VERSION")),
        )
        .position(
            Vector2::x() * ctx.size().x + Vector2::new(-margin, margin) * ctx.scale_factor,
            Anchor::BottomRight,
        )
        .scale(Vector2::repeat(3.0))
        .color(Rgb::repeat(0.5))
        .default_shadow()
        .draw(ctx);

        // Buttons
        let mut root = RootLayout::new(
            Vector2::new(ctx.center().x, ctx.size().y * 0.45),
            Anchor::Center,
        );

        let buttons: [(_, fn(&mut App, &mut _)); _] = [
            ("Campaign", |s, _| s.push_screen(CampaignScreen::default())),
            ("Sandbox", |s, _| s.push_screen(SandboxScreen::default())),
            ("Options", |_, modal| *modal = ActiveModal::Settings),
            ("About", |_, modal| *modal = ActiveModal::About),
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
                        .default_shadow()
                        .button(key)
                        .effects(ButtonEffects::Color | ButtonEffects::Arrows);

                    button
                        .is_clicked(ctx)
                        .then(|| on_click(state, &mut self.modal));
                    button.layout(ctx, layout);
                    Spacer::new_y(5.0 * ctx.scale_factor - padding).layout(ctx, layout);
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
    fn modals(&mut self, state: &mut App, ctx: &mut GraphicsContext) {
        match self.modal {
            ActiveModal::None => {}
            ActiveModal::Settings => self.settings_modal(state, ctx),
            ActiveModal::About => self.about_modal(state, ctx),
        }
    }

    fn settings_modal(&mut self, state: &mut App, ctx: &mut GraphicsContext) {
        let (margin, padding) = state.spacing(ctx);
        let modal = Modal::new(state.modal_size(ctx))
            .position(ctx.center(), Anchor::Center)
            .margin(margin)
            .layer(layer::OVERLAY);

        let size = modal.inner_size();
        modal.draw(ctx, |ctx, root| {
            let body = body(size.x);

            root.nest(ctx, ColumnLayout::new(padding), |ctx, layout| {
                body("Options")
                    .scale(Vector2::repeat(4.0))
                    .layout(ctx, layout);
                Spacer::new_y(4.0 * ctx.scale_factor).layout(ctx, layout);

                layout.nest(ctx, RowLayout::new(padding * 4.0), |ctx, layout| {
                    layout.nest(ctx, ColumnLayout::new(padding), |ctx, layout| {
                        checkbox(ctx, layout, &mut state.config.vsync, "Use VSync");
                        checkbox(ctx, layout, &mut state.config.show_fps, "Show FPS");
                        checkbox(ctx, layout, &mut state.config.debug, "Debug Mode");
                    });

                    layout.nest(ctx, ColumnLayout::new(padding), |ctx, layout| {
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

                let clicking = ctx.input.mouse_down(MouseButton::Left);
                let (back, _) = modal_buttons(ctx, layout, size.x, ("Back", ""));
                (back && clicking).then(|| self.modal = ActiveModal::None);
            });
        });
    }

    fn about_modal(&mut self, state: &mut App, ctx: &mut GraphicsContext) {
        let (margin, padding) = state.spacing(ctx);
        let desired_size = state.modal_size(ctx);

        let description = Text::new(UNDEAD_FONT, DESCRIPTION)
            .max_width(desired_size.x - margin * 2.0)
            .scale(Vector2::repeat(2.0));
        let height = description.size(ctx).y;

        let modal = Modal::new(Vector2::new(
            desired_size.x,
            height + 100.0 * ctx.scale_factor,
        ))
        .position(ctx.center(), Anchor::Center)
        .margin(margin)
        .layer(layer::OVERLAY);

        let size = modal.inner_size();
        modal.draw(ctx, |ctx, root| {
            let body = body(size.x);

            root.nest(ctx, ColumnLayout::new(padding), |ctx, layout| {
                body("About")
                    .scale(Vector2::repeat(4.0))
                    .layout(ctx, layout);
                Spacer::new_y(4.0 * ctx.scale_factor).layout(ctx, layout);

                description.layout(ctx, layout);

                let clicking = ctx.input.mouse_down(MouseButton::Left);
                let (back, _) = modal_buttons(ctx, layout, size.x, ("Back", ""));
                (back && clicking).then(|| self.modal = ActiveModal::None);
            });
        });
    }
}

impl Default for TitleScreen {
    fn default() -> Self {
        Self {
            modal: ActiveModal::None,
        }
    }
}
