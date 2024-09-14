use nalgebra::Vector2;
use winit::dpi::PhysicalSize;

#[derive(Debug)]
pub struct InputManager {
    window_size: Vector2<u32>,
    pub(crate) mouse: Vector2<f32>,
}

impl InputManager {
    pub fn new(window_size: PhysicalSize<u32>) -> Self {
        Self {
            window_size: Vector2::new(window_size.width, window_size.height),
            mouse: Vector2::new(0.0, 0.0),
        }
    }

    pub(crate) fn update_window_size(&mut self, size: PhysicalSize<u32>) {
        self.window_size = Vector2::new(size.width, size.height);
    }

    pub(crate) fn mouse_move(&mut self, x: f32, y: f32) {
        self.mouse = Vector2::new(x, self.window_size.y as f32 - y);
    }
}
