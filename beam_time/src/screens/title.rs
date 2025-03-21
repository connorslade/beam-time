use std::time::Instant;

use engine::{
    color::OkLab,
    drawable::{rectangle::Rectangle, sprite::Sprite},
    exports::nalgebra::Vector2,
    graphics_context::{Anchor, GraphicsContext},
    screens::Screen,
};

use crate::{
    assets::{ABOUT_BUTTON, CAMPAIGN_BUTTON, COPYRIGHT, SANDBOX_BUTTON, TITLE},
    consts::BACKGROUND_COLOR,
    ui::{
        button::{Button, ButtonState},
        waterfall::Waterfall,
    },
    App,
};

use super::{about::AboutScreen, campaign::CampaignScreen, sandbox::SandboxScreen};

pub struct TitleScreen {
    start_time: Instant,

    sandbox_button: ButtonState,
    campaign_button: ButtonState,
    options_button: ButtonState,
    about_button: ButtonState,
}

impl Screen<App> for TitleScreen {
    fn render(&mut self, state: &mut App, ctx: &mut GraphicsContext<App>) {
        ctx.background(BACKGROUND_COLOR);

        let t = state.start.elapsed().as_secs_f32();
        ctx.draw(
            Rectangle::new(Vector2::new(100.0, 100.0))
                .position(
                    ctx.center() + 50.0 * Vector2::new(t.cos(), t.sin()),
                    Anchor::Center,
                )
                .color(OkLab::new(0.8, 0.1893, 0.0).hue_shift(t))
                .z_index(-1),
        );

        ctx.draw(Waterfall::new(&mut state.waterfall));

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
        let campaign_button = Button::new(CAMPAIGN_BUTTON, &mut self.campaign_button)
            .pos(ctx.center(), Anchor::Center)
            .scale(Vector2::repeat(4.0));
        if campaign_button.is_clicked(ctx) {
            ctx.push_screen(CampaignScreen::default())
        }

        let sandbox_button = Button::new(SANDBOX_BUTTON, &mut self.sandbox_button)
            .pos(
                ctx.center() - Vector2::new(0.0, 14.0 * 5.0 * ctx.scale_factor),
                Anchor::Center,
            )
            .scale(Vector2::repeat(4.0));
        if sandbox_button.is_clicked(ctx) {
            ctx.push_screen(SandboxScreen::default());
        }

        let about_button = Button::new(ABOUT_BUTTON, &mut self.about_button)
            .pos(
                ctx.center() - Vector2::new(0.0, 2.0 * 14.0 * 5.0 * ctx.scale_factor),
                Anchor::Center,
            )
            .scale(Vector2::repeat(4.0));
        if about_button.is_clicked(ctx) {
            ctx.push_screen(AboutScreen::default())
        }

        ctx.draw([campaign_button, sandbox_button, about_button]);
    }

    fn on_init(&mut self, _state: &mut App) {
        self.sandbox_button.reset();
        self.about_button.reset();
        self.options_button.reset();
    }

    fn on_resize(&mut self, state: &mut App, _old_size: Vector2<f32>, _size: Vector2<f32>) {
        state.waterfall.reset();
    }
}

impl Default for TitleScreen {
    fn default() -> Self {
        Self {
            start_time: Instant::now(),

            campaign_button: ButtonState::default(),
            sandbox_button: ButtonState::default(),
            about_button: ButtonState::default(),
            options_button: ButtonState::default(),
        }
    }
}
