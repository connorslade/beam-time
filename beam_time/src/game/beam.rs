use engine::{
    drawable::{sprite::Sprite, text::Text},
    exports::nalgebra::Vector2,
    graphics_context::{Anchor, GraphicsContext},
};

use crate::assets::{BEAM_HORIZONTAL, BEAM_VERTICAL, UNDEAD_FONT};

pub struct BeamState {
    board: Vec<Beam>,
    size: Vector2<usize>,
}

#[derive(Debug, Default, Clone, Copy)]
struct Beam {
    powered: bool,
    direction: Direction,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum Direction {
    #[default]
    Up,
    Right,
    Down,
    Left,
}

impl BeamState {
    pub fn new(size: Vector2<usize>) -> Self {
        let board = vec![Beam::default(); size.x * size.y];
        Self { board, size }
    }

    pub fn tick(&mut self) {
        let to_index = |x: usize, y: usize| y * self.size.x + x;

        // debug
        self.board[0] = Beam {
            powered: true,
            direction: Direction::Down,
        };

        let mut working_board = self.board.clone();

        // Update beams
        for y in 0..self.size.y {
            for x in 0..self.size.x {
                let pos = Vector2::new(x, y);
                let index = y * self.size.x + x;
                let beam = self.board[index];

                if !working_board[index].powered {
                    working_board[index] = beam;
                }

                if let Some(source) = beam.direction.opposite().offset(self.size, pos) {
                    if !self.board[to_index(source.x, source.y)].powered {
                        working_board[index].powered = false;
                    }
                }

                if beam.powered {
                    if let Some(sink) = beam.direction.offset(self.size, pos) {
                        working_board[to_index(sink.x, sink.y)] = Beam {
                            powered: true,
                            direction: beam.direction,
                        };
                    }
                }
            }
        }

        self.board = working_board;
    }

    pub fn render<App>(&mut self, ctx: &mut GraphicsContext<App>) {
        let tile_size = 16.0 * 4.0 * ctx.scale_factor;
        let size = self.size.map(|x| x as f32) * tile_size;

        for x in 0..self.size.x {
            for y in 0..self.size.y {
                let index = y * self.size.x + x;
                let beam = self.board[index];

                let pos = ctx.center() - Vector2::new(x as f32 * tile_size, y as f32 * tile_size)
                    + size / 2.0
                    - Vector2::repeat(tile_size / 2.0);

                if beam.powered {
                    let texture =
                        if beam.direction == Direction::Up || beam.direction == Direction::Down {
                            BEAM_VERTICAL
                        } else {
                            BEAM_HORIZONTAL
                        };
                    let sprite = Sprite::new(texture)
                        .scale(Vector2::repeat(4.0), Anchor::Center)
                        .position(pos, Anchor::Center);
                    ctx.draw(sprite);

                    ctx.draw(
                        Text::new(UNDEAD_FONT, &format!("{:?}", beam.direction))
                            .scale(Vector2::repeat(2.0))
                            .pos(pos, Anchor::Center)
                            .z_index(5),
                    );
                }
            }
        }
    }
}

impl Direction {
    pub fn offset(&self, size: Vector2<usize>, pos: Vector2<usize>) -> Option<Vector2<usize>> {
        let new_pos = match self {
            Direction::Up => Vector2::new(pos.x as i32, pos.y as i32 - 1),
            Direction::Right => Vector2::new(pos.x as i32 + 1, pos.y as i32),
            Direction::Down => Vector2::new(pos.x as i32, pos.y as i32 + 1),
            Direction::Left => Vector2::new(pos.x as i32 - 1, pos.y as i32),
        };

        (new_pos.x >= 0 && new_pos.y >= 0 && new_pos.x < size.x as i32 && new_pos.y < size.y as i32)
            .then(|| new_pos.map(|x| x as usize))
    }

    pub fn opposite(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Right => Self::Left,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
        }
    }
}
