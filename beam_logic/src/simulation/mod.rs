use common::direction::Direction;

pub mod level_state;
pub mod runtime;
pub mod state;
pub mod tick;
pub mod tile;

const MIRROR_REFLECTIONS: [Direction; 4] = [
    Direction::Left,
    Direction::Down,
    Direction::Right,
    Direction::Up,
];
