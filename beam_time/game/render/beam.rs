use beam_logic::simulation::{state::BeamState, tile::BeamTile};
use common::direction::Direction;
use engine::{
    assets::SpriteRef,
    drawable::{Anchor, Drawable, sprite::Sprite},
    exports::nalgebra::Vector2,
    graphics_context::GraphicsContext,
};

use crate::{
    app::App,
    assets::{
        BEAM_FULL_HORIZONTAL, BEAM_FULL_VERTICAL, BEAM_REFLECT_DOWN_LEFT, BEAM_REFLECT_DOWN_RIGHT,
        BEAM_REFLECT_UP_LEFT, BEAM_REFLECT_UP_RIGHT, BEAM_SPLIT_DOWN, BEAM_SPLIT_LEFT,
        BEAM_SPLIT_RIGHT, BEAM_SPLIT_UP,
    },
    consts::layer,
    game::{pancam::Pancam, render::tile::HALF_BEAM},
};

pub trait BeamStateRender {
    fn render(&mut self, ctx: &mut GraphicsContext, state: &App, pancam: &Pancam);
}

const MIRROR_TEXTURES: [SpriteRef; 4] = [
    BEAM_REFLECT_UP_LEFT,
    BEAM_REFLECT_DOWN_RIGHT,
    BEAM_REFLECT_UP_RIGHT,
    BEAM_REFLECT_DOWN_LEFT,
];

const SPLITTER_TEXTURES: [SpriteRef; 4] = [
    BEAM_SPLIT_RIGHT,
    BEAM_SPLIT_UP,
    BEAM_SPLIT_LEFT,
    BEAM_SPLIT_DOWN,
];

impl BeamStateRender for BeamState {
    /// Renders the beam over the board.
    fn render(&mut self, ctx: &mut GraphicsContext, state: &App, pancam: &Pancam) {
        let half_tile = Vector2::repeat(pancam.scale * 16.0 / 2.0);
        let size = ctx.size() + half_tile;

        let origin = pancam.origin_tile();
        let frame = state.frame() as u32;

        for (pos, tile) in self.board.iter() {
            let pos = pos + origin;
            let render_pos = pancam.render_pos(pos.x as usize, pos.y as usize);

            if render_pos.x < -half_tile.x
                || render_pos.y < -half_tile.y
                || render_pos.x > size.x
                || render_pos.y > size.y
            {
                continue;
            }

            let sprite = |texture: SpriteRef| {
                Sprite::new(texture)
                    .uv_offset(Vector2::new(16 * frame as i32, 0))
                    .scale(Vector2::repeat(pancam.scale))
                    .position(render_pos, Anchor::Center)
            };

            // Play animation in reverse if beam is traveling in a direction
            // opposite to the animation frames
            let beam = |direction: Direction| {
                let texture = match direction {
                    Direction::Left | Direction::Right => BEAM_FULL_HORIZONTAL,
                    Direction::Up | Direction::Down => BEAM_FULL_VERTICAL,
                };
                let frame = match direction {
                    Direction::Left | Direction::Down => 2 - frame,
                    Direction::Right | Direction::Up => frame,
                };
                sprite(texture).uv_offset(Vector2::new(16 * frame as i32, 0))
            };

            match tile {
                BeamTile::Beam { direction, .. } => beam(direction).draw(ctx),
                BeamTile::CrossBeam { directions } => {
                    directions.iter().for_each(|&x| beam(x).draw(ctx))
                }
                BeamTile::Mirror {
                    galvoed,
                    powered,
                    direction,
                } => {
                    for (idx, _) in powered.iter().enumerate().filter(|x| x.1.is_some()) {
                        let dir = direction ^ galvoed.any();
                        let texture = MIRROR_TEXTURES[idx + dir as usize * 2];
                        sprite(texture)
                            .z_index(layer::LASER * (idx == 1) as i16)
                            .draw(ctx);
                    }
                }
                BeamTile::Splitter { direction, powered } => {
                    for powered in powered.iter() {
                        let index = (powered as usize + direction as usize * 2) % 4;
                        sprite(SPLITTER_TEXTURES[index])
                            .z_index(layer::LASER)
                            .draw(ctx);
                    }
                }
                BeamTile::Delay {
                    powered,
                    last_powered,
                } => {
                    for (idx, set) in [powered, last_powered].into_iter().enumerate() {
                        for dir in set.iter() {
                            sprite(HALF_BEAM[dir.opposite_if(idx > 0) as usize]).draw(ctx);
                        }
                    }
                }
                BeamTile::Galvo { powered, .. }
                | BeamTile::Wall { powered }
                | BeamTile::Detector { powered } => {
                    for dir in powered.iter() {
                        let layer = if dir == Direction::Down {
                            layer::UNDER_LASER
                        } else {
                            layer::LASER
                        };
                        sprite(HALF_BEAM[dir as usize]).z_index(layer).draw(ctx);
                    }
                }
                _ => {}
            }
        }
    }
}
