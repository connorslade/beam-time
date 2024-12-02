mod consts;
pub mod sprite;

#[repr(C)]
#[derive(Default, Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 4],
    pub uv: [f32; 2],
}

impl Vertex {
    pub const fn new(position: [f32; 4], uv: [f32; 2]) -> Self {
        Vertex { position, uv }
    }
}

#[macro_export]
macro_rules! include_shader {
    ($shader:literal) => {
        wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(include_str!(concat!("shaders/", $shader)).into()),
        }
    };
}
