use std::{
    fs::{self, File},
    path::PathBuf,
    time::Instant,
};

use anyhow::Result;
use bincode::Options;
use chrono::{DateTime, Utc};
use engine::{
    drawable::sprite::Sprite,
    exports::{
        nalgebra::Vector2,
        winit::{event::MouseButton, keyboard::KeyCode},
    },
    graphics_context::{Anchor, GraphicsContext},
};
use log::{info, trace, warn};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    app::App,
    assets::{EMPTY_TILE_A, EMPTY_TILE_B, PERMANENT_TILE_A, PERMANENT_TILE_B},
    consts::{layer, AUTOSAVE_INTERVAL},
    misc::map::Map,
    util::{in_bounds, key_events},
};

use super::{
    beam::{state::BeamState, tile::BeamTile},
    history::History,
    holding::Holding,
    level::Level,
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
    pub level: Option<&'static Level>,

    save_path: Option<PathBuf>,
    selection: SelectionState,

    open_timestamp: Instant,
    last_save: Instant,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct BoardMeta {
    pub version: u32,

    pub name: String,
    pub level: Option<LevelMeta>,
    pub size: Option<Vector2<u32>>,

    pub last_played: DateTime<Utc>,
    pub playtime: u64,
}

#[derive(Default, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct LevelMeta {
    pub id: Uuid,
    pub solved: bool,
}

impl Board {
    pub fn load(path: &PathBuf) -> Result<Self> {
        info!("Loading board from {path:?}");

        let file = File::open(path)?;
        let mut board = bincode::DefaultOptions::new()
            .with_varint_encoding()
            .deserialize_from::<_, Board>(file)?;
        board.transient.save_path = Some(path.to_path_buf());

        trace!("{:?}", board.meta);
        Ok(board)
    }

    pub fn load_meta(path: &PathBuf) -> Result<BoardMeta> {
        let file = File::open(path)?;
        let meta = bincode::DefaultOptions::new()
            .with_varint_encoding()
            .allow_trailing_bytes()
            .deserialize_from::<_, BoardMeta>(file)?;
        Ok(meta)
    }

    pub fn save(mut self, path: &PathBuf) -> Result<()> {
        self.meta.playtime += self.transient.open_timestamp.elapsed().as_secs();
        self.meta.last_played = Utc::now();
        self.meta.version = 3;

        let start = Instant::now();
        info!("Saving board to {path:?}");
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let file = File::create(path)?;
        bincode::DefaultOptions::new()
            .with_varint_encoding()
            .serialize_into(file, &self)?;

        info!("Save took {:?}", start.elapsed());
        Ok(())
    }
}

impl Board {
    fn is_permanent(&self, pos: &Vector2<i32>) -> bool {
        self.transient.level.map(|x| x.permanent.contains(pos)) == Some(true)
    }

    fn tick_autosave(&mut self) {
        if let Some(path) = &self.transient.save_path {
            if self.transient.last_save.elapsed() >= AUTOSAVE_INTERVAL {
                trace!("Autosaving...");
                self.transient.last_save = Instant::now();
                // run async if causing issues
                if let Err(err) = self.clone().save(path) {
                    warn!("Autosave failure: {err}");
                }
            }
        }
    }

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
                    .scale(Vector2::repeat(shared.scale), Anchor::Center)
                    .position(render_pos, Anchor::Center)
                    .z_index(layer::TILE_BACKGROUND);

                // let offset = 7.0 * shared.scale * ctx.scale_factor;
                // let offset = Vector2::new(offset, -offset);
                // ctx.draw(
                //     Text::new(UNDEAD_FONT, "A")
                //         .scale(Vector2::repeat(shared.scale / 2.0))
                //         .pos(render_pos + offset, Anchor::BottomRight)
                //         .z_index(layer::TILE_BACKGROUND_OVERLAY),
                // );

                if !is_empty {
                    let sprite = sim
                        .as_ref()
                        .and_then(|x| x.board.get(pos).base_sprite(frame))
                        .unwrap_or_else(|| tile.asset());

                    let sprite = sprite
                        .scale(Vector2::repeat(shared.scale), Anchor::Center)
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
                        key_events!(ctx, {
                            KeyCode::KeyR => {
                                self.tiles.set(pos, tile.rotate());
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

impl Default for TransientBoardState {
    fn default() -> Self {
        Self {
            holding: Default::default(),
            history: History::new(),
            level: None,

            save_path: None,
            selection: Default::default(),

            open_timestamp: Instant::now(),
            last_save: Instant::now(),
        }
    }
}

impl Clone for Board {
    fn clone(&self) -> Self {
        Self {
            meta: self.meta.clone(),
            tiles: self.tiles.clone(),
            transient: TransientBoardState::default(),
        }
    }
}
