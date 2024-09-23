use engine::{
    assets::SpriteRef,
    drawable::sprite::Sprite,
    exports::{nalgebra::Vector2, winit::event::MouseButton},
    graphics_context::{Anchor, GraphicsContext},
    screens::Screen,
};

use crate::{
    assets::EMPTY_TILE,
    consts::{FOREGROUND_COLOR, PLAYER_TILES},
    ui::{button::ButtonState, misc::titled_screen},
    App,
};

pub struct LevelsScreen {
    holding: Option<SpriteRef>,
    tiles: Vec<SpriteRef>,
    size: (usize, usize),

    back_button: ButtonState,
}

impl Screen<App> for LevelsScreen {
    fn render(&mut self, state: &mut App, ctx: &mut GraphicsContext<App>) {
        titled_screen(state, ctx, &mut self.back_button, "Levels");
        self.tile_picker(ctx);
        self.tile_map(ctx);
    }
}

impl LevelsScreen {
    fn tile_picker(&mut self, ctx: &mut GraphicsContext<App>) {
        if ctx.input.mouse_down(MouseButton::Right) {
            self.holding = None;
        }

        if let Some(holding) = self.holding {
            ctx.draw(
                Sprite::new(holding)
                    .scale(Vector2::repeat(4.0), Anchor::Center)
                    .position(ctx.input.mouse, Anchor::Center)
                    .color(FOREGROUND_COLOR)
                    .z_index(10),
            );
        }

        let tile_size = 16.0 * 4.0 * ctx.scale_factor;
        for (i, &tile) in PLAYER_TILES.iter().enumerate() {
            let pos = Vector2::new(10.0, 10.0 + tile_size * i as f32);
            let sprite = Sprite::new(tile)
                .position(pos, Anchor::BottomLeft)
                .scale(Vector2::repeat(4.0), Anchor::Center)
                .color(FOREGROUND_COLOR);

            if ctx.input.mouse_down(MouseButton::Left) && sprite.is_hovered(ctx) {
                self.holding = Some(tile);
            }

            ctx.draw(sprite);
        }
    }

    fn tile_map(&mut self, ctx: &mut GraphicsContext<App>) {
        let tile_size = 16.0 * 4.0 * ctx.scale_factor;
        let size = (
            // todo make vec2
            self.size.0 as f32 * tile_size,
            self.size.1 as f32 * tile_size,
        );

        for y in 0..self.size.1 {
            for x in 0..self.size.0 {
                let texture = self.tiles[y * self.size.0 + x];
                let pos = ctx.center()
                    - Vector2::new(
                        x as f32 * tile_size - size.0 / 2.0,
                        y as f32 * tile_size - size.0 / 2.0,
                    );

                let sprite = Sprite::new(texture)
                    .scale(Vector2::repeat(4.0), Anchor::Center)
                    .position(pos, Anchor::Center);

                if ctx.input.mouse_down(MouseButton::Left) && sprite.is_hovered(ctx) {
                    if let Some(holding) = self.holding.take() {
                        self.tiles[y * self.size.0 + x] = holding;
                    } else if self.holding.is_none() {
                        self.holding = Some(texture);
                        self.tiles[y * self.size.0 + x] = EMPTY_TILE;
                    }
                }

                ctx.draw(sprite);
            }
        }
    }
}

impl Default for LevelsScreen {
    fn default() -> Self {
        Self {
            holding: None,
            tiles: vec![EMPTY_TILE; 64],
            size: (8, 8),

            back_button: ButtonState::default(),
        }
    }
}
