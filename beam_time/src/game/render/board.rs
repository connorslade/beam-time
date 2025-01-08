use crate::assets::{HISTOGRAM_MARKER, UNDEAD_FONT};
use crate::game::board::Board;
use crate::{
    app::App,
    assets::{EMPTY_TILE_A, EMPTY_TILE_B, PERMANENT_TILE_A, PERMANENT_TILE_B},
    consts::layer,
    game::{holding::Holding, shared_state::SharedState},
    ui::misc::tile_label,
    util::key_events,
};
use beam_logic::simulation::{state::BeamState, tile::BeamTile};
use common::misc::in_bounds;
use engine::drawable::text::Text;
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
        ctx: &mut GraphicsContext<App>,
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
        self.transient.selection.update(
            ctx,
            shared,
            sim,
            &mut self.tiles,
            &mut self.transient.history,
            &mut self.transient.holding,
        );

        if sim.is_none()
            && ctx.input.key_down(KeyCode::ControlLeft)
            && ctx.input.key_pressed(KeyCode::KeyZ)
        {
            self.transient.history.pop(&mut self.tiles);
        }

        const MESSAGE: &str = "This is a test of the new label system I am considering adding to Beam time to improve the user experance of sandbox worlds.";
        for (pos, title, note) in [(Vector2::new(10.0, 10.0), "Little Note", MESSAGE)] {
            let mut pos = shared.world_to_screen_space(ctx, pos);
            let marker_pos = pos - Vector2::y() * 4.0 * 2.0 * ctx.scale_factor;

            ctx.draw(
                Sprite::new(HISTOGRAM_MARKER)
                    .scale(Vector2::repeat(2.0))
                    .position(marker_pos, Anchor::TopCenter)
                    .z_index(layer::OVERLAY),
            );

            if shared.scale >= 6.0 {
                let text = Text::new(UNDEAD_FONT, note)
                    .max_width(16.0 * 20.0 * ctx.scale_factor)
                    .scale(Vector2::repeat(2.0))
                    .position(pos, Anchor::BottomCenter)
                    .z_index(layer::OVERLAY);
                pos.y += text.size(ctx).y + 8.0 * ctx.scale_factor;
                ctx.draw(text);
            }

            ctx.draw(
                Text::new(UNDEAD_FONT, title)
                    .scale(Vector2::repeat(2.0))
                    .position(pos, Anchor::BottomCenter)
                    .z_index(layer::OVERLAY),
            );
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

                self.transient
                    .selection
                    .update_tile(ctx, shared, hovered, pos, render_pos);

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

                if let Some(label) = self.transient.level.and_then(|x| x.labels.get(&pos)) {
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

                if shift_down || permanent {
                    ctx.draw(grid);
                    continue;
                }

                // TODO: Move to holding.rs?
                if sim.is_none() && grid.is_hovered(ctx) {
                    let holding = &mut self.transient.holding;

                    if ctx.input.mouse_pressed(MouseButton::Left) {
                        let old = tile;
                        match holding {
                            Holding::None if !is_empty => {
                                self.transient.history.track_one(pos, old);
                                self.tiles.remove(pos);
                                *holding = Holding::Tile(tile);
                            }
                            Holding::Tile(tile) => {
                                self.transient.history.track_one(pos, old);
                                self.tiles.set(pos, *tile);
                                *holding = if old.is_empty() {
                                    Holding::None
                                } else {
                                    Holding::Tile(old)
                                };
                            }
                            Holding::Paste(vec) => {
                                let mut old = Vec::new();
                                for (paste_pos, paste_tile) in vec.iter() {
                                    let pos = pos + *paste_pos;
                                    old.push((pos, self.tiles.get(pos)));
                                    self.tiles.set(pos, *paste_tile);
                                }

                                self.transient.history.track_many(old);
                                *holding = Holding::None;
                            }
                            _ => {}
                        }
                    }

                    if ctx.input.mouse_down(MouseButton::Right) {
                        self.tiles.remove(pos);

                        if !tile.is_empty() {
                            self.transient.history.track_one(pos, tile);
                        }
                    }

                    if holding.is_none() {
                        key_events!(ctx, {
                            KeyCode::KeyR => {
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
