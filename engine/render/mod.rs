pub mod shape;
pub mod sprite;

#[macro_export]
macro_rules! include_shader {
    ($shader:literal) => {
        wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(include_str!($shader).into()),
        }
    };
}

pub fn layer_to_z_coord(layer: i16) -> f32 {
    (i16::MAX as f32 - layer as f32) / (i16::MAX as f32 * 2.0)
}
