use std::mem;

use crate::game::board::Board;
use crate::{
    app::App,
    assets::{EMPTY_TILE_A, EMPTY_TILE_B, PERMANENT_TILE_A, PERMANENT_TILE_B},
    consts::layer,
    game::{holding::Holding, shared_state::SharedState},
    ui::misc::tile_label,
    util::key_events,
};
use beam_logic::level::ElementLocation;
use beam_logic::simulation::{state::BeamState, tile::BeamTile};
use beam_logic::tile::Tile;
use common::misc::in_bounds;
use engine::{
    drawable::sprite::Sprite,
    exports::{
        nalgebra::Vector2,
        winit::{event::MouseButton, keyboard::KeyCode},
    },
    graphics_context::{Anchor, GraphicsContext},
};

use super::tile::{BeamTileBaseSprite, TileAsset};

impl Board {
    pub fn render(
        &mut self,
        ctx: &mut GraphicsContext,
        state: &App,
        shared: &SharedState,
        sim: &mut Option<BeamState>,
    ) {
        self.tick_autosave();

        let tile_size = 16.0 * shared.scale * ctx.scale_factor;
        let half_tile = Vector2::repeat(tile_size / 2.0);

        let tile_counts = shared.tile_counts(ctx.size());
        let frame = state.frame();

        let shift_down = ctx.input.key_down(KeyCode::ShiftLeft);
        self.transient.holding.render(ctx, shared);
        self.update_selection(ctx, shared, sim);
        self.render_notes(ctx, state, shared);

        if sim.is_none()
            && ctx.input.key_down(KeyCode::ControlLeft)
            && ctx.input.key_pressed(KeyCode::KeyZ)
        {
            self.transient.history.pop(&mut self.tiles);
        }

        for x in 0..tile_counts.x {
            for y in 0..tile_counts.y {
                let render_pos = shared.render_pos(ctx, (x, y));
                let pos = shared.tile_pos(ctx, (x, y));

                if let Some(size) = self.meta.size {
                    if pos.x < 0 || pos.y < 0 || pos.x as u32 >= size.x || pos.y as u32 >= size.y {
                        continue;
                    }
                }

                let hovered = in_bounds(
                    ctx.input.mouse,
                    (render_pos - half_tile, render_pos + half_tile),
                );

                self.tile_selection(ctx, shared, hovered, pos, render_pos);

                let tile = self.tiles.get(pos);
                let permanent = self.is_permanent(&pos);
                let is_empty = tile.is_empty();

                let grid_color = (pos.x.abs() + pos.y.abs()) as usize % 2;
                let grid_tile = [
                    [EMPTY_TILE_A, EMPTY_TILE_B],
                    [PERMANENT_TILE_A, PERMANENT_TILE_B],
                ][permanent as usize][grid_color];
                let grid = Sprite::new(grid_tile)
                    .scale(Vector2::repeat(shared.scale))
                    .position(render_pos, Anchor::Center)
                    .z_index(layer::TILE_BACKGROUND);

                let element = if let Tile::Detector { id: Some(id) }
                | Tile::Emitter { id: Some(id), .. } = tile
                {
                    ElementLocation::Dynamic(id)
                } else {
                    ElementLocation::Static(pos)
                };
                if let Some(label) = self.transient.level.and_then(|x| x.labels.get(&element)) {
                    let label = tile_label(ctx, shared.scale, render_pos, label);
                    ctx.draw(label.z_index(layer::OVERLAY));
                }

                if !is_empty {
                    let sprite = sim
                        .as_ref()
                        .and_then(|x| x.board.get(pos).base_sprite(frame))
                        .unwrap_or_else(|| tile.asset());

                    let sprite = sprite
                        .scale(Vector2::repeat(shared.scale))
                        .position(render_pos, Anchor::Center);

                    if ctx.input.key_pressed(KeyCode::KeyE) && hovered {
                        if let Some(sim) = sim {
                            if let (BeamTile::Emitter { active, .. }, true) =
                                (sim.board.get_mut(pos), sim.level.is_none())
                            {
                                *active ^= true;
                            }
                        }
                    }

                    ctx.draw(sprite);
                }

                if sim.is_none() && hovered && !shift_down {
                    if ctx.input.mouse_pressed(MouseButton::Left) {
                        let old = tile;
                        match mem::take(&mut self.transient.holding) {
                            Holding::None if !is_empty && !permanent => {
                                self.transient.history.track_one(pos, old);
                                self.tiles.remove(pos);
                                self.transient.holding = Holding::Tile(tile);
                            }
                            Holding::Tile(tile) if !permanent => {
                                self.transient.history.track_one(pos, old);
                                self.tiles.set(pos, tile);

                                if !old.is_empty() {
                                    self.transient.holding = Holding::Tile(old);
                                }
                            }
                            Holding::Paste(tiles) => {
                                let mut old = Vec::new();

                                let is_mutable = |paste_pos: Vector2<i32>| {
                                    self.transient
                                        .level
                                        .map(|x| x.permanent.contains(&(paste_pos + pos)))
                                        != Some(true)
                                };
                                for (paste_pos, paste_tile) in
                                    tiles.iter().filter(|(paste_pos, _)| is_mutable(*paste_pos))
                                {
                                    let pos = pos + paste_pos;
                                    old.push((pos, self.tiles.get(pos)));
                                    self.tiles.set(pos, *paste_tile);
                                }

                                self.transient.history.track_many(old);
                            }
                            x => self.transient.holding = x,
                        }
                    }

                    if !permanent && ctx.input.mouse_down(MouseButton::Right) && !tile.is_empty() {
                        self.tiles.remove(pos);
                        self.transient.history.track_one(pos, tile);
                    }

                    let holding = &mut self.transient.holding;
                    if holding.is_none() {
                        key_events!(ctx, {
                            KeyCode::KeyR => if !permanent {
                                if ctx.input.key_down(KeyCode::ShiftLeft) {
                                    self.tiles.set(pos, tile.rotate_reverse());
                                } else {
                                    self.tiles.set(pos, tile.rotate());
                                }
                                self.transient.history.track_one(pos, tile);
                            },
                            KeyCode::KeyE => {
                                self.tiles.set(pos, tile.activate());
                                self.transient.history.track_one(pos, tile);
                            }
                        });
                    }

                    if !is_empty && ctx.input.key_pressed(KeyCode::KeyQ) {
                        *holding = Holding::Tile(tile);
                    }
                }

                ctx.draw(grid);
            }
        }
    }
}
