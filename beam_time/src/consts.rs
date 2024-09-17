use engine::color::{OkLab, Rgb};

pub const DEFAULT_SIZE: (u32, u32) = (800, 600);
pub const BACKGROUND_COLOR: Rgb<f32> = Rgb::new(0.2941, 0.1843, 0.2235);
pub const FOREGROUND_COLOR: Rgb<f32> = Rgb::new(0.7961, 0.8588, 0.9882);
pub const START_COLOR: OkLab<f32> = OkLab::new(0.773, 0.1131, 0.0);
