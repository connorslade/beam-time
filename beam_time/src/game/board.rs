use std::{fs, path::PathBuf, time::Instant};

use anyhow::Result;
use chrono::{DateTime, Utc};
use engine::{
    drawable::sprite::Sprite,
    exports::{
        nalgebra::Vector2,
        winit::{event::MouseButton, keyboard::KeyCode},
    },
    graphics_context::{Anchor, GraphicsContext},
};
use log::{info, trace};
use serde::{Deserialize, Serialize};

use crate::{
    app::App,
    assets::{EMPTY_TILE_A, EMPTY_TILE_B, PERMANENT_TILE},
    consts::layer,
    misc::map::Map,
    util::in_bounds,
};

use super::{
    beam::{tile::BeamTile, BeamState},
    history::History,
    holding::Holding,
    selection::SelectionState,
    tile::Tile,
    SharedState,
};

#[derive(Default, Serialize, Deserialize)]
pub struct Board {
    pub meta: BoardMeta,
    pub tiles: Map<Tile>,

    #[serde(skip)]
    pub transient: TransientBoardState,
}

pub struct TransientBoardState {
    pub holding: Holding,
    pub history: History,

    open_timestamp: Instant,
    selection: SelectionState,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct BoardMeta {
    pub version: u32,
    pub name: String,
    pub size: Option<Vector2<u32>>,
    pub last_played: DateTime<Utc>,
    pub playtime: u64,
}

impl Board {
    pub fn load(path: &PathBuf) -> Result<Self> {
        info!("Loading board from {path:?}");
        let raw = fs::read(path)?;
        let board = bincode::deserialize::<Board>(&raw)?;
        trace!("{:?}", board.meta);
        Ok(board)
    }

    pub fn save(mut self, path: &PathBuf) -> Result<()> {
        self.meta.playtime += self.transient.open_timestamp.elapsed().as_secs();
        self.meta.last_played = Utc::now();

        info!("Saving board to {path:?}");
        let raw = bincode::serialize(&self)?;
        fs::write(path, raw)?;
        Ok(())
    }

    pub fn render(
        &mut self,
        ctx: &mut GraphicsContext<App>,
        state: &App,
        shared: &SharedState,
        sim: &mut Option<BeamState>,
    ) {
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

        for x in 0..tile_counts.x {
            for y in 0..tile_counts.y {
                let render_pos = shared.render_pos(ctx, (x, y));
                let pos = shared.tile_pos(ctx, (x, y));

                if let Some(size) = self.meta.size {
                    if pos.x < 0 || pos.y < 0 || pos.x as u32 > size.x || pos.y as u32 > size.y {
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

                    if ctx.input.key_pressed(KeyCode::KeyE) && hovered {
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
                                .scale(Vector2::repeat(shared.scale), Anchor::Center)
                                .position(render_pos, Anchor::Center)
                                .z_index(layer::TILE_BACKGROUND_OVERLAY),
                        );
                    }
                }

                if shift_down || !tile.moveable() {
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
                                *holding = Holding::None;
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
                        if ctx.input.key_pressed(KeyCode::KeyR) {
                            self.tiles.set(pos, tile.rotate());
                            self.transient.history.track_one(pos, tile);
                        }

                        if ctx.input.key_pressed(KeyCode::KeyE) {
                            self.tiles.set(pos, tile.activate());
                            self.transient.history.track_one(pos, tile);
                        }
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

impl Default for TransientBoardState {
    fn default() -> Self {
        Self {
            open_timestamp: Instant::now(),
            holding: Default::default(),
            history: History::new(),
            selection: Default::default(),
        }
    }
}
