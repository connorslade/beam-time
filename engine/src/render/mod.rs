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
