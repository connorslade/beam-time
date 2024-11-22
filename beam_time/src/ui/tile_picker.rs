use engine::{
    drawable::{sprite::Sprite, text::Text},
    exports::{
        nalgebra::Vector2,
        winit::{event::MouseButton, keyboard::KeyCode},
    },
    graphics_context::{Anchor, GraphicsContext},
};

use crate::{
    app::App,
    assets::{TILE_PICKER_CENTER, TILE_PICKER_LEFT, TILE_PICKER_RIGHT, UNDEAD_FONT},
    consts::layer,
    game::{holding::Holding, tile::Tile},
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
        holding: &mut Holding,
    ) {
        if !self.update_offset(ctx, sim) {
            return;
        }

        let scale = state.config.ui_scale * 4.0;
        let tile_size = scale * ctx.scale_factor * 16.0;
        for (i, (tile, key)) in Tile::DEFAULT.iter().zip(TILE_SHORTCUTS).enumerate() {
            if !sim && ctx.input.key_pressed(key) {
                *holding = Holding::Tile(*tile);
            }

            let render_tile = match tile {
                x @ Tile::Emitter { .. } | x @ Tile::Galvo { .. } => &x.rotate(),
                x => x,
            };
            let (asset, name) = (render_tile.asset(), tile.name());
            let pos = Vector2::new(tile_size * i as f32, -self.offset);

            let background_texture = if i == 0 {
                TILE_PICKER_LEFT
            } else if i == Tile::DEFAULT.len() - 1 {
                TILE_PICKER_RIGHT
            } else {
                TILE_PICKER_CENTER
            };

            let background = Sprite::new(background_texture)
                .position(pos, Anchor::BottomLeft)
                .scale(Vector2::repeat(scale), Anchor::Center)
                .z_index(layer::UI_BACKGROUND);
            ctx.draw(background);

            let sprite = asset
                .position(pos, Anchor::BottomLeft)
                .scale(Vector2::repeat(scale), Anchor::Center)
                .z_index(layer::UI_ELEMENT);

            if !sim && sprite.is_hovered(ctx) {
                if holding.is_none() {
                    let text = Text::new(UNDEAD_FONT, name)
                        .position(ctx.input.mouse, Anchor::BottomLeft)
                        .scale(Vector2::repeat(2.0 * state.config.ui_scale))
                        .z_index(layer::TILE_HOLDING);
                    ctx.draw(text);
                }

                if ctx.input.mouse_pressed(MouseButton::Left) {
                    *holding = Holding::Tile(*tile);
                }
            }

            ctx.draw(sprite);
        }

        let bounds = (
            Vector2::zeros(),
            Vector2::new(Tile::DEFAULT.len() as f32 * tile_size, tile_size),
        );
        if in_bounds(ctx.input.mouse, bounds) {
            ctx.input.cancel_mouse();
        }
    }

    fn update_offset<App>(&mut self, ctx: &GraphicsContext<App>, sim: bool) -> bool {
        self.offset += ctx.delta_time * 750.0 * if sim { 1.0 } else { -1.0 };

        let max_offset = 16.0 * 4.0 * ctx.scale_factor;
        self.offset = self.offset.clamp(0.0, max_offset);
        self.offset <= max_offset
    }
}
