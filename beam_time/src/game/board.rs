use engine::{
    drawable::sprite::Sprite,
    exports::{
        nalgebra::Vector2,
        winit::{event::MouseButton, keyboard::KeyCode},
    },
    graphics_context::{Anchor, GraphicsContext},
};

use crate::{
    app::App,
    assets::{EMPTY_TILE_A, EMPTY_TILE_B, PERMANENT_TILE},
    consts::layer,
};

use super::{beam::BeamState, tile::Tile, SharedState};

pub struct Board {
    pub tiles: Vec<Tile>,
    pub size: Vector2<usize>,
}

impl Board {
    pub fn new(size: Vector2<usize>) -> Self {
        Self {
            tiles: vec![Tile::Empty; size.x * size.y],
            size,
        }
    }

    pub fn render(
        &mut self,
        ctx: &mut GraphicsContext<App>,
        state: &App,
        shared: &SharedState,
        sim: &mut Option<BeamState>,
        holding: &mut Option<Tile>,
    ) {
        let frame = state.frame();
        for x in 0..self.size.x {
            for y in 0..self.size.y {
                let pos = shared.tile_pos(ctx, self.size, Vector2::new(x, y));
                let index = y * self.size.x + x;
                let tile = self.tiles[index];
                let is_empty = tile.is_empty();

                let grid_tile = [EMPTY_TILE_A, EMPTY_TILE_B][(x + y) % 2];
                let grid = Sprite::new(grid_tile)
                    .scale(Vector2::repeat(shared.scale), Anchor::Center)
                    .position(pos, Anchor::Center)
                    .z_index(layer::TILE_BACKGROUND);

                if !is_empty {
                    let sprite = sim
                        .as_ref()
                        .and_then(|x| x.board[index].base_sprite(frame))
                        .unwrap_or_else(|| Sprite::new(tile.asset()));

                    let sprite = sprite
                        .scale(Vector2::repeat(shared.scale), Anchor::Center)
                        .position(pos, Anchor::Center);

                    if ctx.input.key_pressed(KeyCode::KeyA) && sprite.is_hovered(ctx) {
                        if let Some(emitter) =
                            sim.as_mut().and_then(|sim| sim.board[index].emitter_mut())
                        {
                            *emitter ^= true;
                        }
                    }

                    ctx.draw(sprite);

                    if tile.permanent() {
                        ctx.draw(
                            Sprite::new(PERMANENT_TILE)
                                .scale(Vector2::repeat(4.0), Anchor::Center)
                                .position(pos, Anchor::Center)
                                .z_index(layer::TILE_BACKGROUND_OVERLAY),
                        );
                    }
                }

                if !tile.moveable() {
                    ctx.draw(grid);
                    continue;
                }

                if sim.is_none() && grid.is_hovered(ctx) {
                    if ctx.input.mouse_pressed(MouseButton::Left) {
                        if let Some(was_holding) = holding.take() {
                            self.tiles[index] = was_holding;
                            if !is_empty {
                                *holding = tile.is_some().then_some(tile);
                            }
                        } else if !is_empty && holding.is_none() {
                            *holding = tile.is_some().then_some(tile);
                            self.tiles[index] = Tile::Empty;
                        }
                    }

                    if ctx.input.mouse_down(MouseButton::Right) {
                        self.tiles[index] = Tile::Empty;
                    }

                    if holding.is_none() {
                        if ctx.input.key_pressed(KeyCode::KeyR) {
                            self.tiles[index] = tile.rotate();
                        }

                        if ctx.input.key_pressed(KeyCode::KeyA) {
                            self.tiles[index] = tile.activate();
                        }
                    }

                    if !is_empty && ctx.input.key_pressed(KeyCode::KeyQ) {
                        holding.replace(tile);
                    }

                    if ctx.input.key_down(KeyCode::KeyW) && holding.take().is_none() {
                        self.tiles[index] = Tile::Empty;
                    }
                }

                ctx.draw(grid);
            }
        }
    }
}
