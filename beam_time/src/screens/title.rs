use std::time::Instant;

use engine::{
    drawable::sprite::Sprite,
    exports::nalgebra::Vector2,
    graphics_context::{Anchor, GraphicsContext},
    screens::Screen,
};

use crate::{
    assets::{ABOUT_BUTTON, COPYRIGHT, OPTIONS_BUTTON, START_BUTTON, TITLE},
    consts::BACKGROUND_COLOR,
    ui::{
        button::{Button, ButtonState},
        waterfall::Waterfall,
    },
    App,
};

use super::{about::AboutScreen, pong::PongScreen};

pub struct TitleScreen {
    start_time: Instant,

    start_button: ButtonState,
    options_button: ButtonState,
    about_button: ButtonState,
}

impl Screen<App> for TitleScreen {
    fn render(&mut self, state: &mut App, ctx: &mut GraphicsContext<App>) {
        ctx.background(BACKGROUND_COLOR);
        ctx.draw(Waterfall::new(&mut state.waterfall));

        // Title & copyright
        let pos = Vector2::new(ctx.size().x / 2.0, ctx.size().y * 0.9);
        let t = self.start_time.elapsed().as_secs_f32().sin() / 20.0;
        ctx.draw(
            Sprite::new(TITLE)
                .position(pos, Anchor::TopCenter)
                .scale(Vector2::repeat(6.0), Anchor::Center)
                .rotate(t, Anchor::Center),
        );

        ctx.draw(
            Sprite::new(COPYRIGHT)
                .position(Vector2::new(ctx.size().x - 10.0, 10.0), Anchor::BottomRight)
                .scale(Vector2::repeat(2.0), Anchor::Center),
        );

        // Buttons
        ctx.draw(
            Button::new(START_BUTTON, &mut self.start_button)
                .pos(ctx.center(), Anchor::Center)
                .scale(Vector2::repeat(4.0))
                .on_click(|ctx| ctx.push_screen(PongScreen::default())),
        );

        ctx.draw(
            Button::new(OPTIONS_BUTTON, &mut self.options_button)
                .pos(
                    ctx.center() - Vector2::new(0.0, 14.0 * 5.0 * ctx.scale_factor),
                    Anchor::Center,
                )
                .scale(Vector2::repeat(4.0)),
        );

        ctx.draw(
            Button::new(ABOUT_BUTTON, &mut self.about_button)
                .pos(
                    ctx.center() - Vector2::new(0.0, 2.0 * 14.0 * 5.0 * ctx.scale_factor),
                    Anchor::Center,
                )
                .scale(Vector2::repeat(4.0))
                .on_click(|ctx| ctx.push_screen(AboutScreen::default())),
        );
    }

    fn on_init(&mut self, _state: &mut App) {
        self.start_button.reset();
        self.about_button.reset();
        self.options_button.reset();
    }

    fn on_resize(&mut self, state: &mut App, _size: Vector2<f32>) {
        state.waterfall.reset();
    }
}

impl Default for TitleScreen {
    fn default() -> Self {
        Self {
            start_time: Instant::now(),

            start_button: ButtonState::default(),
            about_button: ButtonState::default(),
            options_button: ButtonState::default(),
        }
    }
}
