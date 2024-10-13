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
    misc::map::Map,
};

use super::{
    beam::{tile::BeamTile, BeamState},
    tile::Tile,
    SharedState,
};

pub struct Board {
    pub tiles: Map<Tile>,
}

impl Board {
    pub fn new() -> Self {
        Self { tiles: Map::new() }
    }

    pub fn render(
        &mut self,
        ctx: &mut GraphicsContext<App>,
        state: &App,
        shared: &SharedState,
        sim: &mut Option<BeamState>,
        holding: &mut Option<Tile>,
    ) {
        let tile_counts = shared.tile_counts(ctx.size());
        let frame = state.frame();

        for x in 0..tile_counts.x {
            for y in 0..tile_counts.y {
                let render_pos = shared.render_pos(ctx, (x, y));
                let pos = shared.tile_pos(ctx, (x, y));

                let tile = self.tiles.get(pos);
                let is_empty = tile.is_empty();

                let grid_tile =
                    [EMPTY_TILE_A, EMPTY_TILE_B][(pos.x.abs() + pos.y.abs()) as usize % 2];
                let grid = Sprite::new(grid_tile)
                    .scale(Vector2::repeat(shared.scale), Anchor::Center)
                    .position(render_pos, Anchor::Center)
                    .z_index(layer::TILE_BACKGROUND);

                if !is_empty {
                    let sprite = sim
                        .as_ref()
                        .and_then(|x| x.board.get(pos).base_sprite(frame))
                        .unwrap_or_else(|| Sprite::new(tile.asset()));

                    let sprite = sprite
                        .scale(Vector2::repeat(shared.scale), Anchor::Center)
                        .position(render_pos, Anchor::Center);

                    if ctx.input.key_pressed(KeyCode::KeyE) && sprite.is_hovered(ctx) {
                        if let Some(BeamTile::Emitter { active, .. }) =
                            sim.as_mut().map(|sim| sim.board.get_mut(pos))
                        {
                            *active ^= true;
                        }
                    }

                    ctx.draw(sprite);

                    if tile.permanent() {
                        ctx.draw(
                            Sprite::new(PERMANENT_TILE)
                                .scale(Vector2::repeat(4.0), Anchor::Center)
                                .position(render_pos, Anchor::Center)
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
                            self.tiles.set(pos, was_holding);
                            if !is_empty {
                                *holding = tile.is_some().then_some(tile);
                            }
                        } else if !is_empty && holding.is_none() {
                            *holding = tile.is_some().then_some(tile);
                            self.tiles.remove(pos);
                        }
                    }

                    if ctx.input.mouse_down(MouseButton::Right) {
                        self.tiles.remove(pos);
                    }

                    if holding.is_none() {
                        if ctx.input.key_pressed(KeyCode::KeyR) {
                            self.tiles.set(pos, tile.rotate());
                        }

                        if ctx.input.key_pressed(KeyCode::KeyE) {
                            self.tiles.set(pos, tile.activate());
                        }
                    }

                    if !is_empty && ctx.input.key_pressed(KeyCode::KeyQ) {
                        holding.replace(tile);
                    }
                }

                ctx.draw(grid);
            }
        }
    }
}
