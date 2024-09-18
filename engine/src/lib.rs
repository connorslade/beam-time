use wgpu::TextureFormat;

pub mod application;
pub mod assets;
pub mod color;
pub mod drawable;
pub mod graphics_context;
pub mod input;
pub mod render;
pub mod screens;

pub mod exports {
    pub use nalgebra;
    pub use winit;
}

pub const TEXTURE_FORMAT: TextureFormat = TextureFormat::Bgra8Unorm;
