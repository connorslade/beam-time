use std::{fs, path::PathBuf, time::Instant};

use anyhow::Result;
use chrono::{DateTime, Utc};
use engine::{
    drawable::sprite::Sprite,
    exports::{
        nalgebra::{Vector, Vector2},
        winit::{event::MouseButton, keyboard::KeyCode},
    },
    graphics_context::{Anchor, GraphicsContext},
};
use log::{info, trace};
use serde::{Deserialize, Serialize};

use crate::{
    app::App,
    assets::{EMPTY_TILE_A, EMPTY_TILE_B, OVERLAY_SELECTION, PERMANENT_TILE},
    consts::layer,
    misc::map::Map,
};

use super::{
    beam::{tile::BeamTile, BeamState},
    tile::Tile,
    SharedState,
};

#[derive(Serialize, Deserialize)]
pub struct Board {
    pub meta: BoardMeta,
    pub tiles: Map<Tile>,

    // Split into transiant board state?
    #[serde(skip, default = "Instant::now")]
    open_timestamp: Instant,
    #[serde(skip)]
    selection: Vec<Vector2<i32>>,
    #[serde(skip)]
    selection_anchor: Option<Vector2<i32>>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct BoardMeta {
    pub name: String,
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
        self.meta.playtime += self.open_timestamp.elapsed().as_secs();
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
                                .scale(Vector2::repeat(shared.scale), Anchor::Center)
                                .position(render_pos, Anchor::Center)
                                .z_index(layer::TILE_BACKGROUND_OVERLAY),
                        );
                    }
                }

                if self.selection.contains(&pos) {
                    let selection_overlay = Sprite::new(OVERLAY_SELECTION)
                        .scale(Vector2::repeat(shared.scale), Anchor::Center)
                        .position(render_pos, Anchor::Center)
                        .z_index(layer::TILE_BACKGROUND_OVERLAY);
                    ctx.draw(selection_overlay);
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

impl Default for Board {
    fn default() -> Self {
        Self {
            meta: Default::default(),
            tiles: Default::default(),
            open_timestamp: Instant::now(),
            selection: Default::default(),
            selection_anchor: Default::default(),
        }
    }
}
