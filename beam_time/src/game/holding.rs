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

#[derive(Default, Copy, Clone, PartialEq, Eq)]
pub enum Holding {
    #[default]
    None,
    Tile(Tile),
}

impl Holding {
    pub fn take(&mut self) -> Option<Tile> {
        if let Holding::Tile(tile) = *self {
            *self = Holding::None;
            Some(tile)
        } else {
            None
        }
    }

    pub fn is_none(&self) -> bool {
        *self == Holding::None
    }

    pub fn render<App>(&mut self, ctx: &mut GraphicsContext<App>, shared: &SharedState) {
        if ctx.input.mouse_down(MouseButton::Right) {
            *self = Holding::None;
        }

        if let Holding::Tile(tile) = self {
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
    }
}
