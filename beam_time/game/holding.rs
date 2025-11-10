use std::collections::BTreeSet;

use beam_logic::{
    level::{ElementLocation, Level},
    tile::Tile,
};
use engine::{
    drawable::sprite::Sprite,
    drawable::{Anchor, Drawable},
    exports::{nalgebra::Vector2, winit::event::MouseButton},
    graphics_context::GraphicsContext,
};

use crate::{
    assets::DYNAMIC_TILE_OUTLINE,
    consts::{keybind, layer},
    game::render::tile::TileAsset,
    ui::misc::tile_label,
    util::key_events,
};

use super::pancam::Pancam;

pub type ClipboardItem = Vec<(Vector2<i32>, Tile)>;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub enum Holding {
    #[default]
    None,
    Tile(Tile),
    Paste(ClipboardItem),
}

impl Holding {
    pub fn is_none(&self) -> bool {
        *self == Holding::None
    }

    pub fn any_dynamic(&self) -> bool {
        match self {
            Holding::None => false,
            Holding::Tile(tile) => tile.id().is_some(),
            Holding::Paste(items) => items.iter().any(|(_p, t)| t.id().is_some()),
        }
    }

    pub fn dynamic(&self) -> BTreeSet<u32> {
        let mut out = BTreeSet::new();
        let mut insert = |id: Option<u32>| {
            if let Some(id) = id {
                out.insert(id);
            }
        };

        match self {
            Holding::None => {}
            Holding::Tile(tile) => insert(tile.id()),
            Holding::Paste(items) => items.iter().for_each(|(_p, t)| insert(t.id())),
        }
        out
    }

    pub fn render(
        &mut self,
        ctx: &mut GraphicsContext,
        pancam: &Pancam,
        level: Option<&'static Level>,
    ) {
        match self {
            Holding::None => {}
            Holding::Tile(tile) => {
                key_events!(ctx, {
                    keybind::ROTATE => {
                        *tile = if ctx.input.key_down(keybind::SHIFT) {
                            tile.rotate_reverse()
                        } else {
                            tile.rotate()
                        };
                    },
                    keybind::TOGGLE => *tile = tile.activate()
                });

                render_tile(ctx, pancam, &level, *tile, ctx.input.mouse());
            }
            Holding::Paste(tiles) => {
                key_events!(ctx, {
                    keybind::ROTATE => {
                        if ctx.input.key_down(keybind::SHIFT) {
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
                    keybind::FLIP_V => if !ctx.input.key_down(keybind::CTRL) {
                        for (pos, tile) in tiles.iter_mut() {
                            *pos = Vector2::new(pos.x, -pos.y);
                            *tile = tile.flip_vertical();
                        }
                    },
                    keybind::FLIP_H => for (pos, tile) in tiles.iter_mut() {
                        *pos = Vector2::new(-pos.x, pos.y);
                        *tile = tile.flip_horizontal();
                    }
                });

                let tile_size = 16.0 * pancam.scale;
                for (pos, tile) in tiles.iter() {
                    let render_pos = ctx.input.mouse() + tile_size * pos.map(|x| x as f32);
                    render_tile(ctx, pancam, &level, *tile, render_pos);
                }
            }
        }

        if (!self.is_none() && ctx.input.consume_mouse_pressed(MouseButton::Right))
            || ctx.input.key_pressed(keybind::PICK)
        {
            match self {
                Holding::Tile(tile) if tile.id().is_none() => *self = Holding::None,
                Holding::Paste(items) => items.retain(|(_p, t)| t.id().is_some()),
                _ => {}
            }
        }
    }
}

fn render_tile(
    ctx: &mut GraphicsContext,
    shared: &Pancam,
    level: &Option<&'static Level>,
    tile: Tile,
    position: Vector2<f32>,
) {
    tile.asset()
        .scale(Vector2::repeat(shared.scale))
        .position(position, Anchor::Center)
        .z_index(layer::TILE_HOLDING)
        .draw(ctx);

    if let Some(id) = tile.id() {
        Sprite::new(DYNAMIC_TILE_OUTLINE)
            .scale(Vector2::repeat(shared.scale))
            .position(position, Anchor::Center)
            .z_index(layer::TILE_HOLDING_BACKGROUND)
            .draw(ctx);

        if let Some(label) = level.and_then(|level| level.labels.get(&ElementLocation::Dynamic(id)))
        {
            tile_label(shared.scale, shared.scale / 2.0, position, label)
                .z_index(layer::TILE_HOLDING_OVERLAY)
                .draw(ctx);
        }
    }
}
