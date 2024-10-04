use bitvec::{bitvec, order::Lsb0, vec::BitVec};
use engine::{
    drawable::sprite::Sprite,
    exports::nalgebra::Vector2,
    graphics_context::{Anchor, GraphicsContext},
};

use crate::assets::BEAM_HORIZONTAL;

pub struct BeamState {
    board: Vec<Beam>,
    size: Vector2<usize>,

    powered: BitVec,
}

#[derive(Default, Clone, Copy)]
struct Beam {
    powered: bool,
    direction: Direction,
}

#[derive(Default, Clone, Copy)]
enum Direction {
    #[default]
    Up,
    Right,
    Down,
    Left,
}

impl BeamState {
    pub fn new(size: Vector2<usize>) -> Self {
        Self {
            board: vec![Beam::default(); size.x * size.y],
            powered: bitvec![0; size.x * size.y],
            size,
        }
    }

    pub fn tick(&mut self) {
        // debug
        self.board[0] = Beam {
            powered: true,
            direction: Direction::Right,
        };

        // Update powered bitset
        for y in 0..self.size.y {
            for x in 0..self.size.x {
                let index = y * self.size.x + x;
                let beam = self.board[index];

                if beam.powered {
                    let pointing = Vector2::new(x as i32, y as i32) + beam.direction.offset();
                    if pointing.x < 0
                        || pointing.y < 0
                        || pointing.x >= self.size.x as i32
                        || pointing.y >= self.size.y as i32
                    {
                        continue;
                    }

                    let index = pointing.y as usize * self.size.x + pointing.x as usize;
                    self.powered.set(index, true);
                }
            }
        }

        // Update beams
        for y in 0..self.size.y {
            for x in 0..self.size.x {
                let index = y * self.size.x + x;
                let beam = self.board[index];
                let neighbor_powered = self.powered[index];

                if index == 0 {
                    continue;
                }

                if beam.powered && !neighbor_powered {
                    self.board[index] = Beam::default();
                } else if !beam.powered && neighbor_powered {
                    self.board[index] = Beam {
                        powered: true,
                        direction: beam.direction,
                    };
                }
            }
        }
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
                    let sprite = Sprite::new(BEAM_HORIZONTAL)
                        .scale(Vector2::repeat(4.0), Anchor::Center)
                        .position(pos, Anchor::Center);
                    ctx.draw(sprite);
                }
            }
        }
    }
}

impl Direction {
    pub fn offset(&self) -> Vector2<i32> {
        match self {
            Self::Up => Vector2::new(0, -1),
            Self::Right => Vector2::new(1, 0),
            Self::Down => Vector2::new(0, 1),
            Self::Left => Vector2::new(-1, 0),
        }
    }
}
