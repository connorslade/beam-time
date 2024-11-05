use engine::{
    assets::SpriteRef,
    drawable::sprite::Sprite,
    exports::nalgebra::Vector2,
    graphics_context::{Anchor, GraphicsContext},
};
use log::trace;
use tile::BeamTile;

use crate::{
    app::App,
    assets::{
        BEAM_FULL_HORIZONTAL, BEAM_FULL_VERTICAL, BEAM_REFLECT_DOWN_LEFT, BEAM_REFLECT_DOWN_RIGHT,
        BEAM_REFLECT_UP_LEFT, BEAM_REFLECT_UP_RIGHT, BEAM_SPLIT_DOWN, BEAM_SPLIT_LEFT,
        BEAM_SPLIT_RIGHT, BEAM_SPLIT_UP,
    },
    consts::{layer, HALF_BEAM},
    misc::{
        direction::{Direction, Directions},
        map::Map,
    },
};

use super::{board::Board, level::Level, tile::Tile, SharedState};

mod tick;
pub mod tile;

const MIRROR_REFLECTIONS: [Direction; 4] = [
    Direction::Left,
    Direction::Down,
    Direction::Right,
    Direction::Up,
];

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

pub struct BeamState {
    pub board: Map<BeamTile>,
    pub level: Option<LevelState>,
}

pub struct LevelState {
    level: &'static Level,
    test_case: usize,
    cooldown: u32,
}

impl BeamState {
    /// Creates a new BeamState from a Board by converting Tiles into their
    /// BeamTile counterparts.
    pub fn new(board: &Board, test: bool) -> Self {
        let level = if test {
            board.transient.level.map(|level| LevelState {
                level,
                test_case: 0,
                cooldown: level.tests.delay.unwrap_or_default(),
            })
        } else {
            None
        };

        let board = board.tiles.map(|x| match x {
            Tile::Empty => BeamTile::Empty,
            Tile::Emitter { rotation, active } => BeamTile::Emitter {
                direction: rotation,
                active,
            },
            Tile::Detector => BeamTile::Detector {
                powered: Directions::empty(),
            },
            Tile::Delay => BeamTile::Delay {
                powered: Directions::empty(),
                last_powered: Directions::empty(),
            },
            Tile::Mirror { rotation } => BeamTile::Mirror {
                direction: rotation,
                original_direction: rotation,
                powered: [None; 2],
            },
            Tile::Splitter { rotation } => BeamTile::Splitter {
                direction: rotation,
                powered: None,
            },
            Tile::Galvo { rotation } => BeamTile::Galvo {
                direction: rotation,
                powered: Directions::empty(),
            },
            Tile::Wall { .. } => BeamTile::Wall {
                powered: Directions::empty(),
            },
        });

        let mut state = Self { board, level };

        if let Some(level) = &mut state.level {
            trace!("Running with level: {}", level.level.name);
            level.setup_case(&mut state.board);
        }

        state
    }

    /// Renders the beam over the board.
    pub fn render(&mut self, ctx: &mut GraphicsContext<App>, state: &App, shared: &SharedState) {
        let half_tile = Vector2::repeat(ctx.scale_factor * shared.scale * 16.0 / 2.0);
        let size = ctx.size() + half_tile;

        let origin = shared.origin_tile(ctx);
        let frame = state.frame() as u32;

        for (pos, tile) in self.board.iter() {
            let pos = pos + origin;
            let render_pos = shared.render_pos(ctx, (pos.x as usize, pos.y as usize));

            if render_pos.x < -half_tile.x
                || render_pos.y < -half_tile.y
                || render_pos.x > size.x
                || render_pos.y > size.y
            {
                continue;
            }

            let sprite = |texture: SpriteRef| {
                Sprite::new(texture)
                    .uv_offset(Vector2::new(16 * frame, 0))
                    .scale(Vector2::repeat(shared.scale), Anchor::Center)
                    .position(render_pos, Anchor::Center)
            };

            match tile {
                BeamTile::Beam {
                    direction: Direction::Left | Direction::Right,
                    ..
                } => ctx.draw(sprite(BEAM_FULL_HORIZONTAL)),
                BeamTile::Beam {
                    direction: Direction::Up | Direction::Down,
                    ..
                } => ctx.draw(sprite(BEAM_FULL_VERTICAL)),
                BeamTile::CrossBeam { .. } => {
                    ctx.draw(sprite(BEAM_FULL_HORIZONTAL));
                    ctx.draw(sprite(BEAM_FULL_VERTICAL));
                }
                BeamTile::Mirror {
                    direction, powered, ..
                } => {
                    for (idx, _) in powered.iter().enumerate().filter(|x| x.1.is_some()) {
                        let texture = MIRROR_TEXTURES[idx + direction as usize * 2];
                        ctx.draw(sprite(texture).z_index(layer::LASER * (idx == 1) as i16));
                    }
                }
                BeamTile::Splitter {
                    direction,
                    powered: Some(powered),
                } => {
                    let index = (powered as usize + direction as usize * 2) % 4;
                    ctx.draw(sprite(SPLITTER_TEXTURES[index]).z_index(layer::LASER));
                }
                BeamTile::Delay {
                    powered,
                    last_powered,
                } => {
                    for (idx, set) in [powered, last_powered].into_iter().enumerate() {
                        for dir in set.iter() {
                            ctx.draw(sprite(HALF_BEAM[opposite_if(dir, idx > 0) as usize]))
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
                        ctx.draw(sprite(HALF_BEAM[dir as usize]).z_index(layer))
                    }
                }
                _ => {}
            }
        }
    }
}

impl LevelState {
    pub fn is_complete(&self) -> bool {
        self.test_case >= self.level.tests.cases.len()
    }

    fn setup_case(&mut self, board: &mut Map<BeamTile>) {
        let tests = &self.level.tests;
        let case = &tests.cases[self.test_case];

        if let Some(cooldown) = tests.delay {
            self.cooldown = cooldown;
        }

        for (pos, state) in tests.lasers.iter().zip(&case.lasers) {
            let pos = pos.into_pos();
            if let BeamTile::Emitter { active, .. } = board.get_mut(pos) {
                *active = *state;
            }
        }
    }

    fn case_correct(&mut self, board: &mut Map<BeamTile>) -> bool {
        if self.cooldown > 0 {
            self.cooldown -= 1;
            return false;
        }

        let tests = &self.level.tests;
        let case = &tests.cases[self.test_case];

        for (pos, state) in tests.detectors.iter().zip(&case.detectors) {
            let pos = pos.into_pos();
            let BeamTile::Detector { powered } = board.get(pos) else {
                return false;
            };

            if powered.any() != *state {
                return false;
            }
        }

        true
    }
}

fn opposite_if(direction: Direction, condition: bool) -> Direction {
    if condition {
        direction.opposite()
    } else {
        direction
    }
}
