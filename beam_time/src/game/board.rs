use engine::{
    drawable::sprite::Sprite,
    exports::{nalgebra::Vector2, winit::event::MouseButton},
    graphics_context::{Anchor, GraphicsContext},
};

use crate::{
    assets::{EMPTY_TILE, EMPTY_TILE_RIGHT, EMPTY_TILE_TOP, EMPTY_TILE_TOP_RIGHT},
    consts::FOREGROUND_COLOR,
};

use super::tile::Tile;

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

    pub fn with(mut self, pos: Vector2<usize>, tile: Tile) -> Self {
        self.tiles[pos.y * self.size.x + pos.x] = tile;
        self
    }

    pub fn render<App>(
        &mut self,
        ctx: &mut GraphicsContext<App>,
        sim: bool,
        holding: &mut Option<Tile>,
    ) {
        let tile_size = 16.0 * 4.0 * ctx.scale_factor;
        let size = self.size.map(|x| x as f32) * tile_size;

        for x in 0..self.size.x {
            for y in 0..self.size.y {
                let tile = self.tiles[y * self.size.x + x];
                let is_empty = tile.is_empty();

                let pos = ctx.center() - Vector2::new(x as f32 * tile_size, y as f32 * tile_size)
                    + size / 2.0
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

                if !is_empty {
                    let sprite = Sprite::new(tile.asset())
                        .scale(Vector2::repeat(4.0), Anchor::Center)
                        .position(pos, Anchor::Center)
                        .rotate(tile.sprite_rotation(), Anchor::Center)
                        .color(FOREGROUND_COLOR);
                    ctx.draw(sprite);
                }

                if !tile.moveable() {
                    ctx.draw(grid);
                    continue;
                }

                let hovered = grid.is_hovered(ctx);
                if !sim && ctx.input.mouse_pressed(MouseButton::Left) && hovered {
                    if let Some(was_holding) = holding.take() {
                        self.tiles[y * self.size.x + x] = was_holding;
                        if !is_empty {
                            *holding = tile.is_some().then_some(tile);
                        }
                    } else if !is_empty && holding.is_none() {
                        *holding = tile.is_some().then_some(tile);
                        self.tiles[y * self.size.x + x] = Tile::Empty;
                    }
                }

                if !sim && ctx.input.mouse_down(MouseButton::Right) && hovered {
                    self.tiles[y * self.size.x + x] = Tile::Empty;
                }

                ctx.draw(grid);
            }
        }
    }
}
