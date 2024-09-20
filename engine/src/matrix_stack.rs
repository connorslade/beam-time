use nalgebra::{Matrix3, Vector2};

pub struct MatrixStack {
    stack: Vec<Matrix3<f32>>,
}

impl MatrixStack {
    pub fn new() -> Self {
        MatrixStack {
            stack: vec![Matrix3::identity()],
        }
    }

    pub fn transform_point(&self, point: Vector2<f32>) -> Vector2<f32> {
        (self.peek() * point.push(1.0)).xy()
    }

    pub fn modify(&mut self, func: impl FnOnce(Matrix3<f32>) -> Matrix3<f32>) {
        self.stack.last_mut().map(|m| *m = func(*m));
    }

    pub fn peek(&self) -> &Matrix3<f32> {
        self.stack.last().unwrap()
    }

    pub fn push(&mut self, matrix: Matrix3<f32>) {
        self.stack.push(matrix * self.peek());
    }

    pub fn dup(&mut self) {
        self.stack.push(*self.peek());
    }

    pub fn pop(&mut self) -> Matrix3<f32> {
        self.stack.pop().unwrap()
    }
}

impl MatrixStack {
    pub fn transform(&mut self, point: Vector2<f32>) -> &mut Self {
        self.modify(|top| Matrix3::new_translation(&point) * top);
        self
    }

    pub fn scale(&mut self, scaling: Vector2<f32>) -> &mut Self {
        self.modify(|top| Matrix3::new_nonuniform_scaling(&scaling) * top);
        self
    }

    pub fn rotate(&mut self, angle: f32) -> &mut Self {
        self.modify(|top| Matrix3::new_rotation(angle) * top);
        self
    }
}
