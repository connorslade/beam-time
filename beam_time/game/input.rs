use std::mem;

use beam_logic::simulation::{state::BeamState, tile::BeamTile};
use engine::{exports::winit::event::MouseButton, graphics_context::GraphicsContext};

use crate::{
    consts::keybind,
    game::{board::Board, holding::Holding, pancam::Pancam},
    util::key_events,
};

impl Board {
    pub fn tick_input(
        &mut self,
        ctx: &mut GraphicsContext,
        pancam: &Pancam,
        sim: &mut Option<BeamState>,
    ) {
        let pos = pancam
            .screen_to_world_space(ctx.input.mouse())
            .map(|x| x.ceil() as i32);
        let tile = self.tiles.get(pos);
        let (empty, permanent, dynamic) = self.tile_props(&tile, &pos);

        // Start selections
        let shift = ctx.input.key_down(keybind::SHIFT);
        let selection = &mut self.transient.selection;
        if selection.selection_start.is_none()
            && shift
            && ctx.input.mouse_pressed(MouseButton::Left)
        {
            selection.selection_start = Some(pos);
        }

        // Toggle emitters
        if ctx.input.key_pressed(keybind::TOGGLE)
            && let Some(sim) = sim
            && let (BeamTile::Emitter { active, .. }, true) =
                (sim.board.get_mut(pos), sim.level.is_none())
        {
            *active ^= true;
        }

        if shift {
            return;
        }

        if ctx.input.mouse_pressed(MouseButton::Left) {
            let old = tile;
            let in_bounds = self.in_bounds(&pos);

            match mem::take(&mut self.transient.holding) {
                Holding::None if !empty && !permanent && in_bounds => {
                    *sim = None;
                    self.transient.history.track_one(pos, old);
                    self.tiles.remove(pos);
                    self.transient.holding = Holding::Tile(tile);
                }
                Holding::Tile(tile) if !permanent && in_bounds => {
                    *sim = None;
                    self.transient.history.track_one(pos, old);
                    self.tiles.set(pos, tile);

                    if (!old.is_empty() && !ctx.input.key_down(keybind::ALT)) || old.id().is_some()
                    {
                        self.transient.holding = Holding::Tile(old);
                    }
                }
                Holding::Paste(tiles) => {
                    *sim = None;
                    let mut old = Vec::new();
                    let mut next = Vec::new();

                    let level = self.transient.level;
                    for set @ (paste_pos, paste_tile) in tiles {
                        let pos = paste_pos + pos;
                        let current_tile = self.tiles.get(pos);
                        if level
                            .map(|x| x.permanent.contains(&pos))
                            .unwrap_or_default()
                            || level.map(|x| x.out_of_bounds(pos)).unwrap_or_default()
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

        self.transient.deleting = (self.transient.deleting
            || ctx.input.mouse_pressed(MouseButton::Right))
            && ctx.input.mouse_down(MouseButton::Right);
        if self.transient.deleting && !permanent && !empty && !dynamic {
            *sim = None;
            self.tiles.remove(pos);
            self.transient.history.track_one(pos, tile);
        }

        let holding = &mut self.transient.holding;
        if holding.is_none() {
            key_events!(ctx, {
                keybind::ROTATE => if !permanent {
                    *sim = None;
                    if shift {
                        self.tiles.set(pos, tile.rotate_reverse());
                    } else {
                        self.tiles.set(pos, tile.rotate());
                    }
                    self.transient.history.track_one(pos, tile);
                },
                keybind::TOGGLE => if sim.is_none() {
                    self.tiles.set(pos, tile.activate());
                    self.transient.history.track_one(pos, tile);
                }
            });
        }

        if !empty && ctx.input.key_pressed(keybind::PICK) {
            *holding = Holding::Tile(tile.generic());
        }
    }
}
