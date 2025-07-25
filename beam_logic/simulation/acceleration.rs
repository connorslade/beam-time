use std::collections::HashSet;

use nalgebra::Vector2;

use crate::tile::TileType;

/// Stores the locations of different component classes to avoid looping through
/// each tile to just update on class.
///
/// The classes:
/// - Delay
/// - Mirror, Splitter
/// - Beam, Cross Beam
/// - Galvo, Emitter, Detector, Wall
pub struct Acceleration {
    pub delay: HashSet<Vector2<i32>>,
    pub beam_modifier: HashSet<Vector2<i32>>,
    pub beam: HashSet<Vector2<i32>>,
    pub post: HashSet<Vector2<i32>>,
}

impl Acceleration {
    pub fn new() -> Self {
        Self {
            delay: HashSet::new(),
            beam_modifier: HashSet::new(),
            beam: HashSet::new(),
            post: HashSet::new(),
        }
    }

    pub fn track(&mut self, tile_type: TileType, position: Vector2<i32>) {
        match tile_type {
            TileType::Delay => {
                self.delay.insert(position);
                self.post.insert(position);
            }
            TileType::Mirror | TileType::Splitter => {
                self.beam_modifier.insert(position);
            }
            TileType::Galvo | TileType::Emitter | TileType::Detector | TileType::Wall => {
                self.post.insert(position);
            }
        }
    }
}
