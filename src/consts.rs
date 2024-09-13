use nalgebra::Vector3;
use wgpu::TextureFormat;

pub const TEXTURE_FORMAT: TextureFormat = TextureFormat::Bgra8Unorm;
pub const DEFAULT_SIZE: (u32, u32) = (800, 600);
pub const BACKGROUND_COLOR: Vector3<f64> = Vector3::new(0.2941, 0.1843, 0.2235);
