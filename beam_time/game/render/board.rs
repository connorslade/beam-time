use std::mem;

use crate::assets::{DYNAMIC_TILE_A, DYNAMIC_TILE_B};
use crate::game::board::Board;
use crate::{
    app::App,
    assets::{EMPTY_TILE_A, EMPTY_TILE_B, PERMANENT_TILE_A, PERMANENT_TILE_B},
    consts::layer,
    game::{holding::Holding, pancam::Pancam},
    ui::misc::tile_label,
    util::key_events,
};
use beam_logic::level::ElementLocation;
use beam_logic::simulation::{state::BeamState, tile::BeamTile};
use common::misc::in_bounds;
use engine::graphics_context::Drawable;
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
        pancam: &Pancam,
        sim: &mut Option<BeamState>,
    ) {
        self.tick_autosave(
            #[cfg(feature = "steam")]
            state,
        );

        let tile_size = 16.0 * pancam.scale * ctx.scale_factor;
        let half_tile = Vector2::repeat(tile_size / 2.0);

        let tile_counts = pancam.tile_counts(ctx.size());
        let frame = state.frame();

        let shift_down = ctx.input.key_down(KeyCode::ShiftLeft);
        self.transient
            .holding
            .render(ctx, pancam, self.transient.level);
        self.update_selection(ctx, pancam, sim);
        self.render_notes(ctx, state, pancam);

        if sim.is_none()
            && ctx.input.key_down(KeyCode::ControlLeft)
            && ctx.input.key_pressed(KeyCode::KeyZ)
        {
            self.transient.history.pop(&mut self.tiles);
        }

        for x in 0..tile_counts.x {
            for y in 0..tile_counts.y {
                let render_pos = pancam.render_pos(ctx, (x, y));
                let pos = pancam.tile_pos(ctx, (x, y));

                if let Some(size) = self.meta.size
                    && (pos.x < 0 || pos.y < 0 || pos.x as u32 >= size.x || pos.y as u32 >= size.y)
                {
                    continue;
                }

                let hovered = in_bounds(
                    ctx.input.mouse,
                    (render_pos - half_tile, render_pos + half_tile),
                );

                self.tile_selection(ctx, pancam, hovered, pos, render_pos);

                let tile = self.tiles.get(pos);
                let empty = tile.is_empty();
                let permanent = self.is_permanent(&pos);
                let dynamic = tile.id().is_some();

                let gridset_index = match (permanent, dynamic) {
                    (true, _) => 1,
                    (false, true) => 2,
                    _ => 0,
                };
                let grid_color = (pos.x.abs() + pos.y.abs()) as usize % 2;
                let grid_tile = [
                    [EMPTY_TILE_A, EMPTY_TILE_B],
                    [PERMANENT_TILE_A, PERMANENT_TILE_B],
                    [DYNAMIC_TILE_A, DYNAMIC_TILE_B],
                ][gridset_index][grid_color];
                let grid = Sprite::new(grid_tile)
                    .scale(Vector2::repeat(pancam.scale))
                    .position(render_pos, Anchor::Center)
                    .z_index(layer::TILE_BACKGROUND);

                let element = tile
                    .id()
                    .map(ElementLocation::Dynamic)
                    .unwrap_or(ElementLocation::Static(pos));
                if let Some(label) = self.transient.level.and_then(|x| x.labels.get(&element)) {
                    let label = tile_label(ctx, pancam.scale, render_pos, label);
                    label.z_index(layer::OVERLAY).draw(ctx);
                }

                if !empty {
                    let sprite = sim
                        .as_ref()
                        .and_then(|x| x.board.get(pos).base_sprite(frame))
                        .unwrap_or_else(|| tile.asset());

                    let sprite = sprite
                        .scale(Vector2::repeat(pancam.scale))
                        .position(render_pos, Anchor::Center);

                    if ctx.input.key_pressed(KeyCode::KeyE)
                        && hovered
                        && let Some(sim) = sim
                        && let (BeamTile::Emitter { active, .. }, true) =
                            (sim.board.get_mut(pos), sim.level.is_none())
                    {
                        *active ^= true;
                    }

                    sprite.draw(ctx);
                }

                // move this out of board render :sob: please
                if sim.is_none() && hovered && !shift_down {
                    if ctx.input.mouse_pressed(MouseButton::Left) {
                        let old = tile;
                        match mem::take(&mut self.transient.holding) {
                            Holding::None if !empty && !permanent => {
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
                                let mut next = Vec::new();

                                let level = self.transient.level;
                                for set @ (paste_pos, paste_tile) in tiles {
                                    let pos = paste_pos + pos;
                                    let current_tile = self.tiles.get(pos);
                                    if level
                                        .map(|x| x.permanent.contains(&pos))
                                        .unwrap_or_default()
                                        || current_tile.id().is_some()
                                    {
                                        next.push(set);
                                        continue;
                                    }

                                    old.push((pos, current_tile));
                                    self.tiles.set(pos, paste_tile);
                                }

                                self.transient.history.track_many(old);
                                if !next.is_empty() {
                                    self.transient.holding = Holding::Paste(next);
                                }
                            }
                            x => self.transient.holding = x,
                        }
                    }

                    if !permanent && ctx.input.mouse_down(MouseButton::Right) && !empty && !dynamic
                    {
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

                    if !empty && ctx.input.key_pressed(KeyCode::KeyQ) {
                        *holding = Holding::Tile(tile.generic());
                    }
                }

                grid.draw(ctx);
            }
        }
    }
}
