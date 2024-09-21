use std::time::Instant;

use engine::{
    assets::AssetRef,
    drawable::sprite::Sprite,
    exports::nalgebra::Vector2,
    graphics_context::{Anchor, GraphicsContext},
    screens::Screen,
};
use rand::{seq::SliceRandom, thread_rng, Rng};

use crate::{
    assets::{ABOUT_BUTTON, COPYRIGHT, OPTIONS_BUTTON, START_BUTTON, TITLE},
    consts::{BACKGROUND_COLOR, LIGHT_BACKGROUND, TILES},
    ui::button::{Button, ButtonState},
};

use super::{about::AboutScreen, pong::PongScreen};

pub struct TitleScreen {
    need_init: bool,
    start_time: Instant,

    // buttons
    start_button: ButtonState,
    options_button: ButtonState,
    about_button: ButtonState,

    // background
    falling_tiles: Vec<FallingTile>,
}

struct FallingTile {
    asset: AssetRef,
    pos: Vector2<f32>,
    vel: f32,
}

impl Screen for TitleScreen {
    fn render(&mut self, ctx: &mut GraphicsContext) {
        ctx.background(BACKGROUND_COLOR);

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

        // Background tiles
        let mut rng = thread_rng();
        let size = ctx.size();
        let tile_offset = 8.0 * 4.0 * ctx.scale_factor;

        while self.falling_tiles.len() < 40 {
            let asset = TILES.choose(&mut rng).unwrap().to_owned();
            let pos = Vector2::new(
                rng.gen::<f32>() * size.x,
                (size.y + tile_offset)
                    * if self.need_init {
                        rng.gen::<f32>()
                    } else {
                        1.0
                    },
            );
            let vel = rng.gen::<f32>() * 50.0 + 100.0;
            self.falling_tiles.push(FallingTile { asset, pos, vel });
        }

        let mut i = 0;
        while i < self.falling_tiles.len() {
            let tile = &mut self.falling_tiles[i];

            ctx.draw(
                Sprite::new(tile.asset)
                    .position(tile.pos, Anchor::Center)
                    .scale(Vector2::repeat(4.0), Anchor::Center)
                    .color(LIGHT_BACKGROUND)
                    .z_index(-10),
            );

            tile.pos.y -= tile.vel * ctx.delta_time;
            if tile.pos.y < -tile_offset {
                self.falling_tiles.remove(i);
            } else {
                i += 1;
            }
        }

        self.need_init = false;
    }

    fn on_init(&mut self) {
        self.start_button.reset();
        self.about_button.reset();
        self.options_button.reset();
    }

    fn on_resize(&mut self, _size: Vector2<f32>) {
        self.falling_tiles.clear();
        self.need_init = true;
    }
}

impl Default for TitleScreen {
    fn default() -> Self {
        Self {
            start_time: Instant::now(),
            need_init: true,

            start_button: ButtonState::default(),
            about_button: ButtonState::default(),
            options_button: ButtonState::default(),

            falling_tiles: Vec::new(),
        }
    }
}
