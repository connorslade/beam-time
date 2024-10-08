use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Not};

use engine::exports::nalgebra::Vector2;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Direction {
    #[default]
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

    pub fn offset(&self, size: Vector2<usize>, pos: Vector2<usize>) -> Option<Vector2<usize>> {
        let new_pos = match self {
            Direction::Up => Vector2::new(pos.x as i32, pos.y as i32 - 1),
            Direction::Right => Vector2::new(pos.x as i32 - 1, pos.y as i32),
            Direction::Down => Vector2::new(pos.x as i32, pos.y as i32 + 1),
            Direction::Left => Vector2::new(pos.x as i32 + 1, pos.y as i32),
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
}

impl Directions {
    pub const fn empty() -> Self {
        Self { inner: 0 }
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
