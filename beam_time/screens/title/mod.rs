use std::mem;

use engine::{
    color::Rgb,
    drawable::{Anchor, Drawable, spacer::Spacer, text::Text},
    exports::{
        nalgebra::Vector2,
        winit::{event::MouseButton, keyboard::KeyCode},
    },
    graphics_context::GraphicsContext,
    layout::{
        Justify, Layout, LayoutElement, LayoutMethods, column::ColumnLayout, root::RootLayout,
        row::RowLayout, tracker::LayoutTracker,
    },
    memory::MemoryKey,
    memory_key,
};

use crate::{
    App,
    assets::{ALAGARD_FONT, UNDEAD_FONT},
    consts::{
        AUTHOR_HOMEPAGE, GAME_HOMEPAGE, WATERFALL, color, keybind, layer,
        spacing::{MARGIN, PADDING},
    },
    ui::{
        components::{
            button::{ButtonEffects, ButtonExt},
            horizontal_rule::Rule,
            modal::{Modal, modal_buttons},
            slider::slider,
            toggle::toggle,
        },
        misc::{body, modal_size, title_layout},
        waterfall::Waterfall,
    },
};

use super::{Screen, campaign::CampaignScreen, sandbox::SandboxScreen};

mod about;

type ButtonCallback = fn(&mut App, &mut ActiveModal);
const BUTTONS: [(&str, KeyCode, ButtonCallback); 4] = [
    ("Campaign", KeyCode::KeyC, |s, _| {
        s.push_screen(CampaignScreen::default())
    }),
    ("Sandbox", KeyCode::KeyS, |s, _| {
        s.push_screen(SandboxScreen::default())
    }),
    ("Options", KeyCode::KeyO, |_, modal| {
        *modal = ActiveModal::Settings
    }),
    ("About", KeyCode::KeyA, |_, modal| {
        *modal = ActiveModal::About
    }),
];

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
        ctx.background(color::BACKGROUND);
        Waterfall::new(WATERFALL).draw(ctx);
        self.modals(state, ctx);

        if ctx.input.consume_key_pressed(keybind::BACK)
            && let ActiveModal::None = mem::take(&mut self.modal)
        {
            ctx.window.close();
        }

        // Title & copyright
        let (scale, pos) = title_layout(ctx, 15.0);
        let title = Text::new(ALAGARD_FONT, "Beam Time")
            .position(pos, Anchor::TopCenter)
            .scale(Vector2::repeat(scale.round()))
            .dark_shadow();
        let size = title.size(ctx);
        title
            .button(memory_key!())
            .effects(ButtonEffects::Arrows | ButtonEffects::Color)
            .on_click(ctx, || {
                let _ = webbrowser::open(GAME_HOMEPAGE);
            })
            .draw(ctx);

        let offset = Vector2::new(size.x / 2.0, -size.y * 1.25);
        Text::new(UNDEAD_FONT, "By Connor Slade")
            .position(pos + offset, Anchor::TopRight)
            .scale(Vector2::repeat((scale / 2.0).round()))
            .dark_shadow()
            .button(memory_key!())
            .effects(ButtonEffects::Arrows | ButtonEffects::Color)
            .on_click(ctx, || {
                let _ = webbrowser::open(AUTHOR_HOMEPAGE);
            })
            .draw(ctx);

        Text::new(
            UNDEAD_FONT,
            format!("v{} - ©2025", env!("CARGO_PKG_VERSION")),
        )
        .position(
            Vector2::x() * ctx.size().x + Vector2::new(-MARGIN, MARGIN),
            Anchor::BottomRight,
        )
        .scale(Vector2::repeat(3.0))
        .color(Rgb::repeat(0.5))
        .dark_shadow()
        .draw(ctx);

        // Buttons
        let mut root = RootLayout::new(
            Vector2::new(ctx.center().x, ctx.size().y * 0.45),
            Anchor::Center,
        );

        root.nest(
            ctx,
            ColumnLayout::new(PADDING).justify(Justify::Center),
            |ctx, layout| {
                Spacer::new_y(60.0).layout(ctx, layout);
                for (name, keycode, on_click) in BUTTONS {
                    let key = memory_key!(name);
                    let button = Text::new(UNDEAD_FONT, name)
                        .scale(Vector2::repeat(4.0))
                        .dark_shadow()
                        .button(key)
                        .effects(ButtonEffects::Color | ButtonEffects::Arrows);

                    (button.is_clicked(ctx) || ctx.input.key_pressed(keycode))
                        .then(|| on_click(state, &mut self.modal));
                    button.layout(ctx, layout);
                    Spacer::new_y(5.0 - PADDING).layout(ctx, layout);
                }
            },
        );

        root.draw(ctx);
    }
}

const SCALE: MemoryKey = memory_key!();

impl TitleScreen {
    fn modals(&mut self, state: &mut App, ctx: &mut GraphicsContext) {
        match self.modal {
            ActiveModal::None => {
                if let Some(&new_scale) = ctx.memory.get(SCALE) {
                    state.config.interface_scale = new_scale;
                }
            }
            ActiveModal::Settings => self.settings_modal(state, ctx),
            ActiveModal::About => self.about_modal(state, ctx),
        }
    }

    fn settings_modal(&mut self, state: &mut App, ctx: &mut GraphicsContext) {
        let modal = Modal::new(modal_size(ctx))
            .position(ctx.center(), Anchor::Center)
            .margin(MARGIN)
            .layer(layer::OVERLAY);

        let size = modal.inner_size();
        modal.draw(ctx, |ctx, root| {
            let body = body(size.x);

            root.nest(ctx, ColumnLayout::new(PADDING), |ctx, layout| {
                body("Options")
                    .scale(Vector2::repeat(4.0))
                    .layout(ctx, layout);
                Spacer::new_y(4.0).layout(ctx, layout);

                layout.nest(ctx, RowLayout::new(PADDING * 2.4), |ctx, layout| {
                    layout.nest(ctx, ColumnLayout::new(PADDING), |ctx, layout| {
                        toggle(ctx, layout, &mut state.config.fullscreen, "Full Screen");
                        Spacer::new_y(4.0).layout(ctx, layout);

                        let tracker = LayoutTracker::new(memory_key!());
                        ColumnLayout::new(PADDING).tracked(tracker).show(
                            ctx,
                            layout,
                            |ctx, layout| {
                                let mut scale = (ctx.memory.get(SCALE).copied())
                                    .unwrap_or(state.config.interface_scale);
                                slider(
                                    (ctx, layout),
                                    "Interface Scale",
                                    &mut scale,
                                    (1.0, 1.0, 2.0),
                                );
                                ctx.memory.insert(SCALE, scale);
                            },
                        );

                        let scale = ctx.memory.get::<f32>(SCALE).unwrap();
                        if *scale != state.config.interface_scale
                            && let Some(bounds) = tracker.bounds(ctx)
                        {
                            body("Close modal for new scale to take effect.")
                                .max_width(bounds.width())
                                .color(color::ERROR)
                                .layout(ctx, layout);
                        }
                    });

                    let rule = |ctx: &mut GraphicsContext, layout: &mut RowLayout| {
                        let height = layout.available().y - 6.0 - PADDING * 2.0;
                        Rule::vertical(height).layout(ctx, layout)
                    };

                    rule(ctx, layout);

                    layout.nest(ctx, ColumnLayout::new(PADDING), |ctx, layout| {
                        slider(
                            (ctx, layout),
                            "Zoom Sensitivity",
                            &mut state.config.zoom_sensitivity,
                            (0.0, 1.0, 2.0),
                        );

                        slider(
                            (ctx, layout),
                            "Movement Speed",
                            &mut state.config.movement_speed,
                            (1000.0, 2000.0, 3000.0),
                        );
                    });

                    rule(ctx, layout);

                    layout.nest(ctx, ColumnLayout::new(PADDING), |ctx, layout| {
                        toggle(ctx, layout, &mut state.config.vsync, "Use VSync");
                        toggle(ctx, layout, &mut state.config.show_fps, "Show FPS");
                        Spacer::new_y(4.0).layout(ctx, layout);
                        toggle(ctx, layout, &mut state.config.debug, "Debug Mode");
                    });
                });

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
