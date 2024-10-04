use engine::{
    assets::SpriteRef,
    drawable::sprite::Sprite,
    exports::nalgebra::Vector2,
    graphics_context::{Anchor, Drawable, GraphicsContext},
};
use rand::{thread_rng, Rng};

use crate::consts::{LIGHT_BACKGROUND, TILES, TILE_ROTATION};

pub struct Waterfall<'a> {
    state: &'a mut WaterfallState,
}

#[derive(Default)]
pub struct WaterfallState {
    tiles: Vec<FallingTile>,
}

struct FallingTile {
    asset: SpriteRef,
    rotation: f32,
    pos: Vector2<f32>,
    vel: f32,
}

impl<'a> Waterfall<'a> {
    pub fn new(state: &'a mut WaterfallState) -> Self {
        Self { state }
    }
}

impl WaterfallState {
    pub fn reset(&mut self) {
        self.tiles.clear();
    }
}

impl<'a, App> Drawable<App> for Waterfall<'a> {
    fn draw(self, ctx: &mut GraphicsContext<App>) {
        let tiles = &mut self.state.tiles;

        let mut rng = thread_rng();
        let size = ctx.size();
        let tile_offset = 8.0 * 4.0 * ctx.scale_factor;

        let is_empty = tiles.is_empty();
        while tiles.len() < 40 {
            let index = rng.gen_range(0..TILES.len());
            let (asset, rotation) = (TILES[index], TILE_ROTATION[index]);
            let pos = Vector2::new(
                rng.gen::<f32>() * size.x,
                (size.y + tile_offset) * if is_empty { rng.gen::<f32>() } else { 1.0 },
            );
            let vel = rng.gen::<f32>() * 50.0 + 100.0;
            let rotation = rng.gen_bool(0.5) as u8 as f32 * rotation;
            tiles.push(FallingTile {
                asset,
                pos,
                rotation,
                vel,
            });
        }

        let mut i = 0;
        while i < tiles.len() {
            let tile = &mut tiles[i];

            ctx.draw(
                Sprite::new(tile.asset)
                    .position(tile.pos, Anchor::Center)
                    .scale(Vector2::repeat(4.0), Anchor::Center)
                    .rotate(tile.rotation, Anchor::Center)
                    .color(LIGHT_BACKGROUND)
                    .z_index(-10),
            );

            tile.pos.y -= tile.vel * ctx.delta_time;
            if tile.pos.y < -tile_offset {
                tiles.remove(i);
            } else {
                i += 1;
            }
        }
    }
}
