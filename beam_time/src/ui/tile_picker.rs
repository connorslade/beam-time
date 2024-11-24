use engine::{
    assets::SpriteRef,
    color::Rgb,
    drawable::{sprite::Sprite, text::Text},
    exports::{
        nalgebra::Vector2,
        winit::{event::MouseButton, keyboard::KeyCode},
    },
    graphics_context::{Anchor, GraphicsContext},
};
use thousands::Separable;

use crate::{
    app::App,
    assets::{
        animated_sprite, TILE_DELAY, TILE_DETECTOR, TILE_EMITTER_RIGHT, TILE_GALVO_RIGHT,
        TILE_MIRROR_A, TILE_PICKER_CENTER, TILE_PICKER_LEFT, TILE_PICKER_RIGHT, TILE_SPLITTER_A,
        TILE_WALL, UNDEAD_FONT,
    },
    consts::layer,
    game::{board::Board, holding::Holding, tile::Tile},
    util::in_bounds,
};

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
    pub fn render(
        &mut self,
        ctx: &mut GraphicsContext<App>,
        state: &App,
        sim: bool,
        board: &mut Board,
    ) {
        if !self.update_offset(ctx, sim) {
            return;
        }

        let scale = state.config.ui_scale * 4.0;
        let tile_size = scale * ctx.scale_factor * 16.0;
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
                .map_or(false, |disabled| disabled.contains(&tile.as_type()));

            if !disabled && !sim && ctx.input.key_pressed(key) {
                board.transient.holding = Holding::Tile(*tile);
            }

            let background_texture = if i == 0 {
                TILE_PICKER_LEFT
            } else if i == Tile::DEFAULT.len() - 1 {
                TILE_PICKER_RIGHT
            } else {
                TILE_PICKER_CENTER
            };

            let background = Sprite::new(background_texture)
                .position(pos, Anchor::BottomLeft)
                .scale(Vector2::repeat(scale))
                .z_index(layer::UI_BACKGROUND);
            let is_hovered = background.is_hovered(ctx);

            let mut sprite = if !matches!(tile, Tile::Wall) && !disabled {
                let frame = state.frame();
                let texture = TILE_ASSETS[tile.as_type() as usize];
                animated_sprite(texture, is_hovered, frame)
            } else {
                tile.asset()
            }
            .position(pos, Anchor::BottomLeft)
            .scale(Vector2::repeat(scale))
            .z_index(layer::UI_ELEMENT);

            if disabled {
                sprite = sprite.color(Rgb::repeat(0.7));
            }

            if !disabled && !sim && is_hovered {
                if board.transient.holding.is_none() {
                    let text = format!("{}\n${}", tile.name(), tile.price().separate_with_commas());
                    let pos = Vector2::new(ctx.input.mouse.x, tile_size * 1.1);
                    let text = Text::new(UNDEAD_FONT, &text)
                        .position(pos, Anchor::BottomCenter)
                        .scale(Vector2::repeat(2.0 * state.config.ui_scale))
                        .z_index(layer::TILE_HOLDING);
                    ctx.draw(text);
                }

                if ctx.input.mouse_pressed(MouseButton::Left) {
                    board.transient.holding = Holding::Tile(*tile);
                }
            }

            ctx.draw(sprite);
            ctx.draw(background);
        }

        let bounds = (
            Vector2::zeros(),
            Vector2::new(Tile::DEFAULT.len() as f32 * tile_size, tile_size),
        );
        if in_bounds(ctx.input.mouse, bounds) {
            ctx.input.cancel_mouse(MouseButton::Left);
            ctx.input.cancel_mouse(MouseButton::Right);
        }
    }

    fn update_offset<App>(&mut self, ctx: &GraphicsContext<App>, sim: bool) -> bool {
        self.offset += ctx.delta_time * 750.0 * if sim { 1.0 } else { -1.0 };

        let max_offset = 16.0 * 4.0 * ctx.scale_factor;
        self.offset = self.offset.clamp(0.0, max_offset);
        self.offset <= max_offset
    }
}
