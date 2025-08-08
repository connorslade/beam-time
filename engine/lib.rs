#![allow(clippy::obfuscated_if_else)]
#![allow(clippy::type_complexity)]

use wgpu::TextureFormat;

pub mod application;
pub mod assets;
pub mod audio;
pub mod color;
pub mod drawable;
pub mod graphics_context;
pub mod layout;
pub mod memory;
pub mod misc;
pub mod render;

pub mod exports {
    pub use nalgebra;
    pub use winit;
}

pub const TEXTURE_FORMAT: TextureFormat = TextureFormat::Bgra8Unorm;
pub const DEPTH_TEXTURE_FORMAT: TextureFormat = TextureFormat::Depth24PlusStencil8;
