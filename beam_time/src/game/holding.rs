use engine::{
    drawable::sprite::Sprite,
    exports::{
        nalgebra::Vector2,
        winit::{event::MouseButton, keyboard::KeyCode},
    },
    graphics_context::{Anchor, GraphicsContext},
};

use crate::consts::layer;

use super::{tile::Tile, SharedState};

#[derive(Default, Clone, PartialEq, Eq)]
pub enum Holding {
    #[default]
    None,
    Tile(Tile),
    Paste(Vec<(Vector2<i32>, Tile)>),
}

impl Holding {
    pub fn is_none(&self) -> bool {
        *self == Holding::None
    }

    pub fn render<App>(&mut self, ctx: &mut GraphicsContext<App>, shared: &SharedState) {
        if ctx.input.mouse_down(MouseButton::Right) || ctx.input.key_pressed(KeyCode::KeyQ) {
            *self = Holding::None;
        }

        match self {
            Holding::None => {}
            Holding::Tile(tile) => {
                if ctx.input.key_pressed(KeyCode::KeyR) {
                    *tile = tile.rotate();
                }

                if ctx.input.key_pressed(KeyCode::KeyE) {
                    *tile = tile.activate();
                }

                ctx.draw(
                    Sprite::new(tile.asset())
                        .scale(Vector2::repeat(shared.scale), Anchor::Center)
                        .position(ctx.input.mouse, Anchor::Center)
                        .z_index(layer::TILE_HOLDING),
                );
            }
            Holding::Paste(tiles) => {
                if ctx.input.key_pressed(KeyCode::KeyR) {
                    for (pos, tile) in tiles.iter_mut() {
                        *pos = Vector2::new(-pos.y, pos.x);
                        *tile = tile.rotate_reverse();
                    }
                }

                let tile_size = 16.0 * shared.scale * ctx.scale_factor;
                for (pos, tile) in tiles.iter() {
                    let render_pos = ctx.input.mouse + tile_size * pos.map(|x| x as f32);
                    ctx.draw(
                        Sprite::new(tile.asset())
                            .scale(Vector2::repeat(shared.scale), Anchor::Center)
                            .position(render_pos, Anchor::Center)
                            .z_index(layer::TILE_HOLDING),
                    );
                }
            }
        }
    }
}
