use engine::{
    assets::SpriteRef,
    drawable::{sprite::Sprite, text::Text},
    exports::{nalgebra::Vector2, winit::event::MouseButton},
    graphics_context::{Anchor, GraphicsContext},
    screens::Screen,
};

use crate::{
    assets::{
        ALAGARD_FONT, EMPTY_TILE, EMPTY_TILE_RIGHT, EMPTY_TILE_TOP, EMPTY_TILE_TOP_RIGHT,
        UNDEAD_FONT,
    },
    consts::{BACKGROUND_COLOR, FOREGROUND_COLOR, PLAYER_TILES, TILE_NAMES},
    App,
};

pub struct LevelsScreen {
    holding: Option<SpriteRef>,
    tiles: Vec<Option<SpriteRef>>,
    size: (usize, usize),
}

impl Screen<App> for LevelsScreen {
    fn render(&mut self, state: &mut App, ctx: &mut GraphicsContext<App>) {
        ctx.background(BACKGROUND_COLOR);

        let money = state.start.elapsed().as_secs_f32().sin() * 600.0 + 600.0;
        ctx.draw(
            Text::new(ALAGARD_FONT, &format!("${money:.0}"))
                .scale(Vector2::repeat(4.0))
                .pos(ctx.center(), Anchor::CenterLeft)
                .color(FOREGROUND_COLOR),
        );

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
        let text_space = 20.0 * ctx.scale_factor;
        for (i, (&tile, name)) in PLAYER_TILES.iter().zip(TILE_NAMES.iter()).enumerate() {
            let pos = Vector2::new(10.0, (tile_size + text_space) * i as f32 + text_space * 2.0);
            let sprite = Sprite::new(tile)
                .position(pos, Anchor::BottomLeft)
                .scale(Vector2::repeat(3.0), Anchor::Center)
                .color(FOREGROUND_COLOR);

            if ctx.input.mouse_pressed(MouseButton::Left) && sprite.is_hovered(ctx) {
                self.holding = Some(tile);
            }

            ctx.draw(sprite);
            ctx.draw(
                Text::new(UNDEAD_FONT, name)
                    .scale(Vector2::repeat(2.0))
                    .pos(pos + Vector2::new(10.0, -10.0), Anchor::TopLeft),
            );
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
                let is_empty = texture.is_none();

                let pos = ctx.center()
                    - Vector2::new(
                        x as f32 * tile_size - size.0 / 2.0,
                        y as f32 * tile_size - size.0 / 2.0,
                    )
                    - Vector2::repeat(tile_size / 2.0);

                let grid_tile = if x == 0 && y == 0 {
                    EMPTY_TILE_TOP_RIGHT
                } else if y == 0 {
                    EMPTY_TILE_TOP
                } else if x == 0 {
                    EMPTY_TILE_RIGHT
                } else {
                    EMPTY_TILE
                };
                let grid = Sprite::new(grid_tile)
                    .scale(Vector2::repeat(4.0), Anchor::Center)
                    .position(pos, Anchor::Center)
                    .z_index(-10);

                if let Some(texture) = texture {
                    let sprite = Sprite::new(texture)
                        .scale(Vector2::repeat(4.0), Anchor::Center)
                        .position(pos, Anchor::Center)
                        .color(FOREGROUND_COLOR);
                    ctx.draw(sprite);
                }

                let hovered = grid.is_hovered(ctx);

                if ctx.input.mouse_pressed(MouseButton::Left) && hovered {
                    if let Some(holding) = self.holding.take() {
                        self.tiles[y * self.size.0 + x] = Some(holding);
                        if !is_empty {
                            self.holding = texture;
                        }
                    } else if !is_empty && self.holding.is_none() {
                        self.holding = texture;
                        self.tiles[y * self.size.0 + x] = None;
                    }
                }

                if ctx.input.mouse_down(MouseButton::Right) && hovered {
                    self.tiles[y * self.size.0 + x] = None;
                }

                ctx.draw(grid);
            }
        }
    }
}

impl Default for LevelsScreen {
    fn default() -> Self {
        Self {
            holding: None,
            tiles: vec![None; 64],
            size: (8, 8),
        }
    }
}
