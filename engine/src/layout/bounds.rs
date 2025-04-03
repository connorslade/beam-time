use std::ops::{Add, AddAssign};

use nalgebra::Vector2;

#[derive(Default, Clone, Copy)]
pub struct Bounds2D {
    pub min: Vector2<f32>,
    pub max: Vector2<f32>,
}

impl Bounds2D {
    pub fn new(min: Vector2<f32>, max: Vector2<f32>) -> Self {
        Bounds2D { min, max }
    }

    pub fn from_points(points: &[Vector2<f32>]) -> Self {
        let mut min = Vector2::new(f32::MAX, f32::MAX);
        let mut max = Vector2::new(f32::MIN, f32::MIN);

        for point in points {
            min.x = min.x.min(point.x);
            min.y = min.y.min(point.y);
            max.x = max.x.max(point.x);
            max.y = max.y.max(point.y);
        }

        Bounds2D { min, max }
    }

    pub fn translate(&mut self, distance: Vector2<f32>) {
        self.min += distance;
        self.max += distance;
    }

    pub fn height(&self) -> f32 {
        self.max.y - self.min.y
    }

    pub fn width(&self) -> f32 {
        self.max.x - self.min.x
    }
}

impl Add for Bounds2D {
    type Output = Bounds2D;

    fn add(self, rhs: Self) -> Self::Output {
        Bounds2D {
            min: Vector2::new(self.min.x.min(rhs.min.x), self.min.y.min(rhs.min.y)),
            max: Vector2::new(self.max.x.max(rhs.max.x), self.max.y.max(rhs.max.y)),
        }
    }
}

impl AddAssign for Bounds2D {
    fn add_assign(&mut self, rhs: Self) {
        self.min.x = self.min.x.min(rhs.min.x);
        self.min.y = self.min.y.min(rhs.min.y);
        self.max.x = self.max.x.max(rhs.max.x);
        self.max.y = self.max.y.max(rhs.max.y);
    }
}
