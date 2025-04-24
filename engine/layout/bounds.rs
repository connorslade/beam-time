use std::ops::{Add, AddAssign};

use nalgebra::Vector2;

use crate::{drawable::shape::rectangle_outline::RectangleOutline, graphics_context::Anchor};

#[derive(Debug, Clone, Copy)]
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

    pub fn outline(&self) -> RectangleOutline {
        RectangleOutline::new(self.size(), 1.0)
            .position(self.min, Anchor::BottomLeft)
            .z_index(i16::MAX)
    }

    pub fn translate(&mut self, distance: Vector2<f32>) {
        self.min += distance;
        self.max += distance;
    }

    pub fn translated(&self, distance: Vector2<f32>) -> Self {
        Bounds2D {
            min: self.min + distance,
            max: self.max + distance,
        }
    }

    pub fn size(&self) -> Vector2<f32> {
        self.max - self.min
    }

    pub fn height(&self) -> f32 {
        self.max.y - self.min.y
    }

    pub fn width(&self) -> f32 {
        self.max.x - self.min.x
    }

    pub fn contains(&self, point: Vector2<f32>) -> bool {
        point.x >= self.min.x
            && point.x <= self.max.x
            && point.y >= self.min.y
            && point.y <= self.max.y
    }
}

impl Default for Bounds2D {
    fn default() -> Self {
        Bounds2D {
            min: Vector2::new(f32::MAX, f32::MAX),
            max: Vector2::new(f32::MIN, f32::MIN),
        }
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
