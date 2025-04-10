use beam_logic::tile::Tile;
use engine::{
    exports::{
        nalgebra::Vector2,
        winit::{event::MouseButton, keyboard::KeyCode},
    },
    graphics_context::{Anchor, GraphicsContext},
};

use crate::{consts::layer, game::render::tile::TileAsset, util::key_events};

use super::shared_state::SharedState;

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

    pub fn render(&mut self, ctx: &mut GraphicsContext, shared: &SharedState) {
        if ctx.input.mouse_down(MouseButton::Right) || ctx.input.key_pressed(KeyCode::KeyQ) {
            *self = Holding::None;
        }

        match self {
            Holding::None => {}
            Holding::Tile(tile) => {
                key_events!(ctx, {
                    KeyCode::KeyR => {
                        *tile = if ctx.input.key_down(KeyCode::ShiftLeft) {
                            tile.rotate_reverse()
                        } else {
                            tile.rotate()
                        };
                    },
                    KeyCode::KeyE => *tile = tile.activate()
                });

                ctx.draw(
                    tile.asset()
                        .scale(Vector2::repeat(shared.scale))
                        .position(ctx.input.mouse, Anchor::Center)
                        .z_index(layer::TILE_HOLDING),
                );
            }
            Holding::Paste(tiles) => {
                key_events!(ctx, {
                    KeyCode::KeyR => {
                        if ctx.input.key_down(KeyCode::ShiftLeft) {
                            for (pos, tile) in tiles.iter_mut() {
                                *pos = Vector2::new(-pos.y, pos.x);
                                *tile = tile.rotate_reverse();
                            }
                        }  else {
                            for (pos, tile) in tiles.iter_mut() {
                                *pos = Vector2::new(pos.y, -pos.x);
                                *tile = tile.rotate();
                            }
                        }
                    },
                    KeyCode::KeyV => for (pos, tile) in tiles.iter_mut() {
                        *pos = Vector2::new(pos.x, -pos.y);
                        *tile = tile.flip_vertical();
                    },
                    KeyCode::KeyH => for (pos, tile) in tiles.iter_mut() {
                        *pos = Vector2::new(-pos.x, pos.y);
                        *tile = tile.flip_horizontal();
                    }
                });

                let tile_size = 16.0 * shared.scale * ctx.scale_factor;
                for (pos, tile) in tiles.iter() {
                    let render_pos = ctx.input.mouse + tile_size * pos.map(|x| x as f32);
                    ctx.draw(
                        tile.asset()
                            .scale(Vector2::repeat(shared.scale))
                            .position(render_pos, Anchor::Center)
                            .z_index(layer::TILE_HOLDING),
                    );
                }
            }
        }
    }
}
