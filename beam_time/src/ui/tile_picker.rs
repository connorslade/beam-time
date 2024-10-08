use engine::{
    drawable::{sprite::Sprite, text::Text},
    exports::{
        nalgebra::Vector2,
        winit::{event::MouseButton, keyboard::KeyCode},
    },
    graphics_context::{Anchor, GraphicsContext},
};

use crate::{assets::UNDEAD_FONT, game::tile::Tile};

pub fn tile_picker<App>(ctx: &mut GraphicsContext<App>, holding: &mut Option<Tile>) {
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
                .scale(Vector2::repeat(4.0), Anchor::Center)
                .position(ctx.input.mouse, Anchor::Center)
                .z_index(10),
        );
    }

    let tile_size = 16.0 * 4.0 * ctx.scale_factor;
    let text_space = 20.0 * ctx.scale_factor;
    for (i, tile) in Tile::DEFAULT.iter().enumerate() {
        let (asset, name) = (tile.asset(), tile.name());

        let pos = Vector2::new(10.0, (tile_size + text_space) * i as f32 + text_space * 2.0);
        let sprite = Sprite::new(asset)
            .position(pos, Anchor::BottomLeft)
            .scale(Vector2::repeat(4.0), Anchor::Center);

        if ctx.input.mouse_pressed(MouseButton::Left) && sprite.is_hovered(ctx) {
            *holding = Some(*tile);
        }

        ctx.draw(sprite);
        ctx.draw(
            Text::new(UNDEAD_FONT, name)
                .scale(Vector2::repeat(2.0))
                .pos(pos + Vector2::new(10.0, -10.0), Anchor::TopLeft),
        );
    }
}
