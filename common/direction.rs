use std::{
    f32::consts::PI,
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Mul, Not},
};

use nalgebra::Vector2;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum Direction {
    #[default]
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Directions {
    inner: u8,
}

impl Direction {
    pub const ALL: [Direction; 4] = [
        Direction::Up,
        Direction::Right,
        Direction::Down,
        Direction::Left,
    ];

    pub fn rotate(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }

    pub fn rotate_reverse(self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Right => Self::Up,
            Self::Down => Self::Right,
            Self::Left => Self::Down,
        }
    }

    pub fn offset(&self, pos: Vector2<i32>) -> Vector2<i32> {
        match self {
            Direction::Up => Vector2::new(pos.x, pos.y + 1),
            Direction::Right => Vector2::new(pos.x + 1, pos.y),
            Direction::Down => Vector2::new(pos.x, pos.y - 1),
            Direction::Left => Vector2::new(pos.x - 1, pos.y),
        }
    }

    pub fn opposite(self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Right => Self::Left,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
        }
    }

    pub fn opposite_if(self, condition: bool) -> Direction {
        if condition { self.opposite() } else { self }
    }

    pub fn flip_vertical(self) -> Self {
        match self {
            Direction::Left | Direction::Right => self,
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
        }
    }

    pub fn flip_horizontal(self) -> Self {
        match self {
            Direction::Up | Direction::Down => self,
            Direction::Right => Direction::Left,
            Direction::Left => Direction::Right,
        }
    }

    pub fn is_perpendicular(self, other: Self) -> bool {
        match self {
            Self::Up | Self::Down => matches!(other, Self::Left | Self::Right),
            Self::Left | Self::Right => matches!(other, Self::Up | Self::Down),
        }
    }

    pub fn is_horizontal(self) -> bool {
        matches!(self, Self::Left | Self::Right)
    }

    pub fn is_vertical(self) -> bool {
        matches!(self, Self::Up | Self::Down)
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

impl Directions {
    pub const fn empty() -> Self {
        Self { inner: 0 }
    }

    pub const fn set(&mut self, direction: Direction, active: bool) {
        let bit = 1 << direction as u8;
        self.inner = (self.inner & !bit) | (bit * active as u8);
    }

    pub const fn contains(&self, direction: Direction) -> bool {
        self.inner & 1 << direction as u8 != 0
    }

    pub const fn any(&self) -> bool {
        self.inner != 0
    }

    pub const fn any_but(&self, direction: Direction) -> bool {
        self.inner & !(1 << direction as u8) != 0
    }

    pub const fn count(&self) -> u32 {
        self.inner.count_ones()
    }

    pub fn iter(self) -> impl Iterator<Item = Direction> {
        Direction::ALL
            .into_iter()
            .filter(move |&x| self.contains(x))
    }
}

impl BitOr<Direction> for Directions {
    type Output = Self;

    fn bitor(self, rhs: Direction) -> Self::Output {
        Self {
            inner: self.inner | 1 << rhs as u8,
        }
    }
}

impl BitOr<Directions> for Directions {
    type Output = Self;

    fn bitor(self, rhs: Directions) -> Self::Output {
        Self {
            inner: self.inner | rhs.inner,
        }
    }
}

impl BitAnd<Direction> for Directions {
    type Output = bool;

    fn bitand(self, rhs: Direction) -> Self::Output {
        self.inner & 1 << rhs as u8 != 0
    }
}

impl Not for Directions {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self {
            inner: !self.inner & 0b1111,
        }
    }
}

impl BitOrAssign<Direction> for Directions {
    fn bitor_assign(&mut self, rhs: Direction) {
        self.inner |= 1 << rhs as u8;
    }
}

impl BitAndAssign<Directions> for Directions {
    fn bitand_assign(&mut self, rhs: Directions) {
        self.inner &= rhs.inner;
    }
}

impl Mul<bool> for Direction {
    type Output = Directions;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn mul(self, rhs: bool) -> Self::Output {
        if rhs {
            Directions::empty() | self
        } else {
            Directions::empty()
        }
    }
}

impl FromIterator<Direction> for Directions {
    fn from_iter<I: IntoIterator<Item = Direction>>(iter: I) -> Self {
        let mut out = Self::empty();
        for i in iter {
            out |= i;
        }

        out
    }
}

impl From<Direction> for Directions {
    fn from(direction: Direction) -> Self {
        Self::empty() | direction
    }
}

impl From<&Direction> for Directions {
    fn from(direction: &Direction) -> Self {
        Self::empty() | *direction
    }
}
