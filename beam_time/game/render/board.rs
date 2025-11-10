use crate::{
    app::App,
    assets::{DYNAMIC_TILE_A, DYNAMIC_TILE_B},
    assets::{EMPTY_TILE_A, EMPTY_TILE_B, PERMANENT_TILE_A, PERMANENT_TILE_B},
    consts::keybind,
    consts::layer,
    game::board::Board,
    game::pancam::Pancam,
    ui::misc::tile_label,
};
use beam_logic::level::ElementLocation;
use beam_logic::simulation::state::BeamState;
use engine::{
    drawable::sprite::Sprite,
    drawable::{Anchor, Drawable},
    exports::nalgebra::Vector2,
    graphics_context::GraphicsContext,
};

use super::tile::{BeamTileBaseSprite, TileAsset};

impl Board {
    pub fn render(
        &mut self,
        ctx: &mut GraphicsContext,
        state: &mut App,
        pancam: &Pancam,
        sim: &mut Option<BeamState>,
    ) {
        self.tick_autosave(state);

        let tile_counts = pancam.tile_counts(ctx.size());
        let frame = state.frame();

        self.update_selection(ctx, state, pancam, sim);
        self.transient
            .holding
            .render(ctx, pancam, self.transient.level);
        self.render_notes(ctx, state, pancam);

        if sim.is_none()
            && ctx.input.key_down(keybind::CTRL)
            && ctx.input.key_pressed(keybind::UNDO)
        {
            self.transient
                .history
                .pop(&mut self.tiles, &self.transient.holding);
        }

        for x in 0..tile_counts.x {
            for y in 0..tile_counts.y {
                let pos = pancam.tile_pos(x as i32, y as i32);
                if !self.in_bounds(&pos) {
                    continue;
                }

                let render_pos = pancam.render_pos(x, y);
                self.tile_selection(ctx, pancam, pos, render_pos);

                let tile = self.tiles.get(pos);
                let (empty, permanent, dynamic) = self.tile_props(&tile, &pos);

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

                let element = (tile.id())
                    .map(ElementLocation::Dynamic)
                    .unwrap_or(ElementLocation::Static(pos));
                if let Some(label) = self.transient.level.and_then(|x| x.labels.get(&element)) {
                    let label = tile_label(pancam.scale, pancam.scale / 2.0, render_pos, label);
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

                    sprite.draw(ctx);
                }

                grid.draw(ctx);
            }
        }
    }
}
