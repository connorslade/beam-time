use std::time::Instant;

use engine::{
    assets::SpriteRef,
    drawable::{Anchor, Drawable},
    exports::nalgebra::Vector2,
    graphics_context::GraphicsContext,
    memory::{Memory, MemoryKey},
};
use rand::{Rng, seq::IndexedRandom};

use crate::{
    assets::{TILE_DELAY, TILE_DETECTOR, TILE_WALL, animated_sprite},
    consts::layer,
    game::render::tile::{EMITTER, GALVO, MIRROR, SPLITTER},
};

const TILES: &[&[SpriteRef]] = &[
    &MIRROR,
    &SPLITTER,
    &[TILE_WALL],
    &GALVO,
    &EMITTER,
    &[TILE_DETECTOR],
    &[TILE_DELAY],
];

pub struct Waterfall {
    key: MemoryKey,
}

struct WaterfallState {
    start: Instant,
    tiles: Vec<FallingTile>,
}

struct FallingTile {
    asset: SpriteRef,
    active: bool,
    pos: Vector2<f32>,
    vel: Vector2<f32>,
}

impl Waterfall {
    pub fn new(key: MemoryKey) -> Self {
        Self { key }
    }
}

impl Drawable for Waterfall {
    fn draw(self, ctx: &mut GraphicsContext) {
        let mut rng = rand::rng();
        let size = ctx.size();
        let tile_offset = 8.0 * 4.0;

        // todo: maybe do smth about this
        let memory: &mut Memory = unsafe { &mut *(ctx.memory as *mut _) };
        let state = memory.get_or_insert_with(self.key, WaterfallState::default);

        let area = size.x * size.y;
        let count = (area.sqrt() / 30.0) as usize;

        let tiles = &mut state.tiles;
        let is_empty = tiles.is_empty();
        while tiles.len() < count {
            let group = TILES.choose(&mut rng).unwrap();
            let asset = *group.choose(&mut rng).unwrap();

            let pos_y = if is_empty || ctx.window.resized().is_some() {
                size.y * rng.random::<f32>()
            } else {
                size.y + tile_offset
            };
            let pos = Vector2::new(rng.random::<f32>(), pos_y);
            let vel = Vector2::y() * -(rng.random::<f32>() * 50.0 + 100.0);
            let active = rng.random::<bool>();
            tiles.push(FallingTile {
                asset,
                active,
                pos,
                vel,
            });
        }

        let frame = state.start.elapsed().as_millis() as u8 / 100 % 3;

        let mut i = 0;
        while i < tiles.len() {
            let tile = &mut tiles[i];

            let pos = Vector2::new(tile.pos.x * size.x, tile.pos.y);
            animated_sprite(tile.asset, tile.active, frame)
                .position(pos, Anchor::Center)
                .scale(Vector2::repeat(4.0))
                .z_index(layer::TILE_BACKGROUND)
                .draw(ctx);

            if !ctx.window.just_focused() {
                tile.pos += tile.vel * ctx.delta_time;
            }

            if tile.pos.y < -tile_offset || i > count {
                tiles.remove(i);
            } else {
                i += 1;
            }
        }
    }
}

impl Default for WaterfallState {
    fn default() -> Self {
        WaterfallState {
            start: Instant::now(),
            tiles: Vec::new(),
        }
    }
}
