use engine::{
    assets::SpriteRef,
    drawable::sprite::Sprite,
    exports::{
        nalgebra::Vector2,
        winit::{event::MouseButton, keyboard::KeyCode},
    },
    graphics_context::{Anchor, GraphicsContext},
};

use crate::{
    assets::{EMPTY_TILE, EMPTY_TILE_RIGHT, EMPTY_TILE_TOP, EMPTY_TILE_TOP_RIGHT},
    consts::FOREGROUND_COLOR,
};

use super::{beam::BeamState, tile::Tile, tile_pos};

const GRID_TILES: [SpriteRef; 4] = [
    EMPTY_TILE,
    EMPTY_TILE_TOP,
    EMPTY_TILE_RIGHT,
    EMPTY_TILE_TOP_RIGHT,
];

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

    pub fn render<App>(
        &mut self,
        ctx: &mut GraphicsContext<App>,
        sim: &mut Option<BeamState>,
        holding: &mut Option<Tile>,
    ) {
        for x in 0..self.size.x {
            for y in 0..self.size.y {
                let pos = tile_pos(ctx, self.size, Vector2::new(x, y));
                let index = y * self.size.x + x;
                let tile = self.tiles[index];
                let is_empty = tile.is_empty();

                let grid_tile = GRID_TILES[(x == 0) as usize * 2 + (y == 0) as usize];
                let grid = Sprite::new(grid_tile)
                    .scale(Vector2::repeat(4.0), Anchor::Center)
                    .position(pos, Anchor::Center)
                    .z_index(-10);

                if !is_empty {
                    // Use rotation from the simulation if it exists, otherwise
                    // use the base tile's rotation.
                    let rotation = sim
                        .as_ref()
                        .and_then(|x| x.board[index].rotation_override())
                        .unwrap_or_else(|| tile.sprite_rotation());

                    let asset = sim
                        .as_ref()
                        .and_then(|x| x.board[index].texture_override())
                        .unwrap_or_else(|| tile.asset());

                    let sprite = Sprite::new(asset)
                        .scale(Vector2::repeat(4.0), Anchor::Center)
                        .position(pos, Anchor::Center)
                        .rotate(rotation, Anchor::Center)
                        .color(FOREGROUND_COLOR);

                    // todo: cleanup
                    if ctx.input.key_pressed(KeyCode::KeyA) && sprite.is_hovered(ctx) {
                        if let Some(sim) = sim {
                            if let Some(emitter) = sim.board[index].emitter_mut() {
                                *emitter ^= true;
                            }
                        }
                    }

                    ctx.draw(sprite);
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
