use crate::misc::direction::Direction;

mod level;
pub mod state;
mod tick;
pub mod tile;

const MIRROR_REFLECTIONS: [Direction; 4] = [
    Direction::Left,
    Direction::Down,
    Direction::Right,
    Direction::Up,
];

fn opposite_if(direction: Direction, condition: bool) -> Direction {
    if condition {
        direction.opposite()
    } else {
        direction
    }
}
