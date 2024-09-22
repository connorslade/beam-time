use nalgebra::Vector2;
use winit::{
    dpi::PhysicalSize,
    event::{ElementState, MouseButton, WindowEvent},
    keyboard::PhysicalKey,
};

#[derive(Debug)]
pub struct InputManager {
    pub(crate) window_size: Vector2<u32>,
    pub mouse: Vector2<f32>,
    pub resized: bool,

    mouse_down: Vec<MouseButton>,
    key_down: Vec<PhysicalKey>,
}

impl InputManager {
    pub fn new(window_size: PhysicalSize<u32>) -> Self {
        Self {
            window_size: Vector2::new(window_size.width, window_size.height),
            mouse: Vector2::new(0.0, 0.0),
            resized: false,
            mouse_down: Vec::new(),
            key_down: Vec::new(),
        }
    }

    pub fn mouse_down(&self, button: MouseButton) -> bool {
        self.mouse_down.contains(&button)
    }

    pub fn key_down(&self, key: PhysicalKey) -> bool {
        self.key_down.contains(&key)
    }

    pub(crate) fn on_window_event(&mut self, window_event: &WindowEvent) {
        self.resized = false;
        match window_event {
            WindowEvent::Resized(size) => {
                self.window_size = Vector2::new(size.width, size.height);
                self.resized = true;
            }
            WindowEvent::CursorMoved { position: pos, .. } => {
                self.mouse = Vector2::new(pos.x as f32, self.window_size.y as f32 - pos.y as f32)
            }
            WindowEvent::MouseInput { state, button, .. } => match state {
                ElementState::Pressed => self.mouse_down.push(button.to_owned()),
                ElementState::Released => {
                    let idx = self.mouse_down.iter().position(|x| x == button);
                    if let Some(idx) = idx {
                        self.mouse_down.remove(idx);
                    }
                }
            },
            WindowEvent::KeyboardInput { event, .. } => match event.state {
                ElementState::Pressed => self.key_down.push(event.physical_key),
                ElementState::Released => {
                    let idx = self.key_down.iter().position(|x| x == &event.physical_key);
                    if let Some(idx) = idx {
                        self.key_down.remove(idx);
                    }
                }
            },
            _ => {}
        }
    }
}
