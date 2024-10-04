use std::f32::consts::PI;

use engine::exports::nalgebra::Vector2;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    #[default]
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn rotate(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }

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

    pub fn opposite(self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Right => Self::Left,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
        }
    }

    pub fn is_perpendicular(self, other: Self) -> bool {
        match self {
            Self::Up | Self::Down => matches!(other, Self::Left | Self::Right),
            Self::Left | Self::Right => matches!(other, Self::Up | Self::Down),
        }
    }

    pub fn to_angle(self) -> f32 {
        match self {
            Self::Up => 0.0,
            Self::Left => PI / 2.0,
            Self::Right => -PI / 2.0,
            Self::Down => PI,
        }
    }
}
