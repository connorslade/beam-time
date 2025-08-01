use ahash::HashMap;
use nalgebra::Vector2;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(transparent)]
pub struct Map<T> {
    pub tiles: HashMap<Vector2<i32>, T>,
}

impl<T: Default + Copy + PartialEq> Map<T> {
    pub fn get(&self, pos: Vector2<i32>) -> T {
        self.tiles.get(&pos).copied().unwrap_or_default()
    }

    pub fn get_mut(&mut self, pos: Vector2<i32>) -> &mut T {
        self.tiles.entry(pos).or_default()
    }

    pub fn set(&mut self, pos: Vector2<i32>, tile: T) {
        if tile == T::default() {
            self.tiles.remove(&pos);
        } else {
            self.tiles.insert(pos, tile);
        }
    }

    pub fn remove(&mut self, pos: Vector2<i32>) {
        self.tiles.remove(&pos);
    }

    pub fn iter(&self) -> impl Iterator<Item = (Vector2<i32>, T)> + '_ {
        self.tiles.iter().map(|(&k, &v)| (k, v))
    }

    pub fn map<U>(&self, mut f: impl FnMut(Vector2<i32>, T) -> U) -> Map<U> {
        Map {
            tiles: self.tiles.iter().map(|(&k, &v)| (k, f(k, v))).collect(),
        }
    }
}

impl<T: Clone> Clone for Map<T> {
    fn clone(&self) -> Self {
        Self {
            tiles: self.tiles.clone(),
        }
    }
}
