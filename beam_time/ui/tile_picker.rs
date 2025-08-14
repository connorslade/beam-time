use common::misc::{exp_decay, in_bounds};
use thousands::Separable;

use crate::{
    app::App,
    assets::{
        TILE_DELAY, TILE_DETECTOR, TILE_EMITTER_RIGHT, TILE_GALVO_RIGHT, TILE_MIRROR_A,
        TILE_SPLITTER_A, TILE_WALL, UNDEAD_FONT, animated_sprite,
    },
    consts::layer,
    game::{board::Board, holding::Holding, render::tile::TileAsset},
};
use beam_logic::tile::Tile;
use engine::{
    assets::SpriteRef,
    color::Rgb,
    drawable::text::Text,
    drawable::{Anchor, Drawable},
    exports::{
        nalgebra::Vector2,
        winit::{event::MouseButton, keyboard::KeyCode},
    },
    graphics_context::GraphicsContext,
};

use super::components::modal::{Modal, ModalSides};

const TILE_SHORTCUTS: [KeyCode; 7] = [
    KeyCode::Digit1,
    KeyCode::Digit2,
    KeyCode::Digit3,
    KeyCode::Digit4,
    KeyCode::Digit5,
    KeyCode::Digit6,
    KeyCode::Digit7,
];

const TILE_ASSETS: [SpriteRef; 7] = [
    TILE_DETECTOR,
    TILE_DELAY,
    TILE_EMITTER_RIGHT,
    TILE_MIRROR_A,
    TILE_SPLITTER_A,
    TILE_GALVO_RIGHT,
    TILE_WALL,
];

#[derive(Default)]
pub struct TilePicker {
    offset: f32,
}

impl TilePicker {
    pub fn render(&mut self, ctx: &mut GraphicsContext, state: &App, sim: bool, board: &mut Board) {
        if !self.update_offset(ctx, sim) {
            return;
        }

        let mouse = ctx.input.mouse();
        let px = 4.0;
        let tile_size = 16.0 * px;

        Modal::new(Vector2::new(tile_size * 7.0 - px * 2.0, tile_size - px))
            .position(Vector2::new(px, -self.offset), Anchor::BottomLeft)
            .sides(ModalSides::TOP | ModalSides::LEFT | ModalSides::RIGHT)
            .layer(layer::UI_BACKGROUND)
            .popup(false)
            .draw_empty(ctx);

        for (i, (tile, key)) in Tile::DEFAULT.iter().zip(TILE_SHORTCUTS).enumerate() {
            let pos = Vector2::new(tile_size * i as f32, -self.offset);
            let tile = match tile {
                Tile::Emitter { .. } | Tile::Galvo { .. } => &tile.rotate(),
                x => x,
            };

            let disabled = board
                .transient
                .level
                .and_then(|x| x.disabled.as_ref())
                .is_some_and(|disabled| disabled.contains(&tile.as_type()));

            let color = if disabled {
                Rgb::repeat(0.7)
            } else {
                Rgb::repeat(1.0)
            };

            if !disabled && !sim && ctx.input.key_pressed(key) {
                board.transient.holding = Holding::Tile(*tile);
            }

            let is_hovered = in_bounds(mouse, (pos, pos + Vector2::repeat(tile_size)));
            if !matches!(tile, Tile::Wall) && !disabled {
                let frame = state.frame();
                let texture = TILE_ASSETS[tile.as_type() as usize];
                let animate = is_hovered && board.transient.holding.is_none();
                animated_sprite(texture, animate, frame)
            } else {
                tile.asset()
            }
            .position(pos, Anchor::BottomLeft)
            .scale(Vector2::repeat(4.0))
            .z_index(layer::UI_ELEMENT)
            .color(color)
            .draw(ctx);

            if !sim && is_hovered {
                if board.transient.holding.is_none() {
                    let text = if disabled {
                        "🔒 Locked".into()
                    } else {
                        format!("{}\n${}", tile.name(), tile.price().separate_with_commas())
                    };

                    let pos = Vector2::new(mouse.x, tile_size * 1.1);
                    Text::new(UNDEAD_FONT, &text)
                        .position(pos, Anchor::BottomCenter)
                        .scale(Vector2::repeat(2.0))
                        .z_index(layer::TILE_HOLDING)
                        .draw(ctx);
                }

                if !disabled && ctx.input.mouse_pressed(MouseButton::Left) {
                    board.transient.holding = Holding::Tile(*tile);
                }
            }
        }

        let bounds = (
            Vector2::zeros(),
            Vector2::new(Tile::DEFAULT.len() as f32 * tile_size, tile_size),
        );
        if in_bounds(mouse, bounds) {
            ctx.input.cancel_clicks();
        }
    }

    fn update_offset(&mut self, ctx: &GraphicsContext, sim: bool) -> bool {
        let max_offset = 16.0 * 4.0;
        let end = max_offset * sim as u8 as f32;

        self.offset = exp_decay(self.offset, end, 10.0, ctx.delta_time);
        self.offset = self.offset.clamp(0.0, max_offset);

        self.offset <= max_offset
    }
}
