use nalgebra::{Scalar, Vector2};

pub fn in_bounds<T: Scalar + PartialOrd>(
    pos: Vector2<T>,
    bounds: (Vector2<T>, Vector2<T>),
) -> bool {
    pos.x >= bounds.0.x && pos.x <= bounds.1.x && pos.y >= bounds.0.y && pos.y <= bounds.1.y
}

pub fn lerp(start: f32, end: f32, t: f32) -> f32 {
    let lerp = start + (end - start) * t;
    lerp.clamp(start.min(end), start.max(end))
}

pub fn exp_decay(start: f32, end: f32, decay: f32, dt: f32) -> f32 {
    let lerp_speed = (-decay * dt).exp();
    lerp(end, start, lerp_speed)
}
