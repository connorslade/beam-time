use engine::{
    drawable::{sprite::Sprite, text::Text},
    exports::{
        nalgebra::Vector2,
        winit::{event::MouseButton, keyboard::KeyCode},
    },
    graphics_context::{Anchor, GraphicsContext},
};

use crate::{
    assets::{
        TILE_PICKER_BACKGROUND_CENTER, TILE_PICKER_BACKGROUND_LEFT, TILE_PICKER_BACKGROUND_RIGHT,
        UNDEAD_FONT,
    },
    consts::layer,
    game::{tile::Tile, SharedState},
};

pub fn tile_picker<App>(
    ctx: &mut GraphicsContext<App>,
    shared: &SharedState,
    holding: &mut Option<Tile>,
) {
    if ctx.input.mouse_down(MouseButton::Right) {
        *holding = None;
    }

    if let Some(holding) = holding {
        if ctx.input.key_pressed(KeyCode::KeyR) {
            *holding = holding.rotate();
        }

        if ctx.input.key_pressed(KeyCode::KeyA) {
            *holding = holding.activate();
        }

        ctx.draw(
            Sprite::new(holding.asset())
                .scale(Vector2::repeat(shared.scale), Anchor::Center)
                .position(ctx.input.mouse, Anchor::Center)
                .z_index(layer::TILE_HOLDING),
        );
    }

    let tile_size = 16.0 * 4.0 * ctx.scale_factor;
    for (i, tile) in Tile::DEFAULT.iter().enumerate() {
        let (asset, name) = (tile.asset(), tile.name());
        let pos = Vector2::new(tile_size * i as f32, 0.0);

        let background_texture = if i == 0 {
            TILE_PICKER_BACKGROUND_LEFT
        } else if i == Tile::DEFAULT.len() - 1 {
            TILE_PICKER_BACKGROUND_RIGHT
        } else {
            TILE_PICKER_BACKGROUND_CENTER
        };

        let background = Sprite::new(background_texture)
            .position(pos, Anchor::BottomLeft)
            .scale(Vector2::repeat(4.0), Anchor::Center);
        ctx.draw(background);

        let sprite = Sprite::new(asset)
            .position(pos, Anchor::BottomLeft)
            .scale(Vector2::repeat(4.0), Anchor::Center);

        if sprite.is_hovered(ctx) {
            if holding.is_none() {
                let text = Text::new(UNDEAD_FONT, name)
                    .pos(ctx.input.mouse, Anchor::BottomLeft)
                    .scale(Vector2::repeat(2.0))
                    .z_index(layer::TILE_HOLDING);
                ctx.draw(text);
            }

            if ctx.input.mouse_pressed(MouseButton::Left) {
                *holding = Some(*tile);
            }
        }

        ctx.draw(sprite);
    }
}
