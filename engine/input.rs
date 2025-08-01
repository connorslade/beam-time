use nalgebra::Vector2;
use winit::{
    dpi::PhysicalSize,
    event::{ElementState, KeyEvent, MouseButton, MouseScrollDelta, WindowEvent},
    keyboard::{KeyCode, PhysicalKey},
};

#[derive(Debug)]
pub struct InputManager {
    pub(crate) window_size: Vector2<u32>,
    pub scale_factor: f32,
    pub mouse: Vector2<f32>,
    pub mouse_delta: Vector2<f32>,
    pub scroll_delta: f32,
    /// Previous size
    pub resized: Option<Vector2<u32>>,
    pub dpi_changed: Option<f32>,

    pub clicks_canceled: bool,
    pub mouse_down: Vec<MouseButton>,
    pub mouse_actions: Vec<(MouseButton, ElementState)>,

    pub key_down: Vec<PhysicalKey>,
    pub key_actions: Vec<KeyEvent>,
}

impl InputManager {
    pub fn new(window_size: PhysicalSize<u32>) -> Self {
        Self {
            window_size: Vector2::new(window_size.width, window_size.height),
            scale_factor: 1.0,
            mouse: Vector2::new(0.0, 0.0),
            mouse_delta: Vector2::new(0.0, 0.0),
            scroll_delta: 0.0,
            resized: None,
            dpi_changed: None,

            clicks_canceled: false,
            mouse_down: Vec::new(),
            mouse_actions: Vec::new(),

            key_down: Vec::new(),
            key_actions: Vec::new(),
        }
    }

    pub fn mouse_down(&self, button: MouseButton) -> bool {
        if self.clicks_canceled {
            return false;
        }

        self.mouse_down.contains(&button)
    }

    pub fn mouse_pressed(&self, button: MouseButton) -> bool {
        self.mouse_actions
            .iter()
            .any(|(b, s)| b == &button && s == &ElementState::Pressed)
    }

    pub fn mouse_released(&self, button: MouseButton) -> bool {
        self.mouse_actions
            .iter()
            .any(|(b, s)| b == &button && s == &ElementState::Released)
    }

    pub fn cancel_clicks(&mut self) {
        self.mouse_actions.clear();
        self.clicks_canceled = true;
    }

    pub fn cancel_hover(&mut self) {
        self.mouse *= -1.0;
    }

    pub fn key_down(&self, key: KeyCode) -> bool {
        self.key_down.contains(&key.into())
    }

    pub fn key_pressed(&self, key: KeyCode) -> bool {
        self.key_actions
            .iter()
            .any(|e| e.state == ElementState::Pressed && e.physical_key == key)
    }

    pub fn consume_key_pressed(&mut self, key: KeyCode) -> bool {
        let idx = self
            .key_actions
            .iter()
            .position(|e| e.state == ElementState::Pressed && e.physical_key == key);

        if let Some(idx) = idx {
            self.key_actions.remove(idx);
        }

        idx.is_some()
    }

    pub fn key_released(&self, key: KeyCode) -> bool {
        self.key_actions
            .iter()
            .any(|e| e.state == ElementState::Released && e.physical_key == key)
    }

    pub fn delta_size(&self) -> Vector2<f32> {
        if let Some(resized) = self.resized {
            self.window_size.map(|x| x as f32) - resized.map(|x| x as f32)
        } else {
            Vector2::zeros()
        }
    }

    pub fn resized(&self) -> bool {
        self.resized.is_some()
    }

    pub fn dpi_changed(&self) -> bool {
        self.dpi_changed.is_some()
    }

    pub(crate) fn on_window_event(&mut self, window_event: &WindowEvent) {
        match window_event {
            WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                self.dpi_changed = Some(self.scale_factor);
                self.scale_factor = *scale_factor as f32;
            }
            WindowEvent::Resized(size) => {
                self.resized = Some(self.window_size);
                self.window_size = Vector2::new(size.width, size.height);
            }
            WindowEvent::CursorMoved { position: pos, .. } => {
                let new_mouse =
                    Vector2::new(pos.x as f32, self.window_size.y as f32 - pos.y as f32);
                self.mouse_delta += new_mouse - self.mouse;
                self.mouse = new_mouse;
            }
            WindowEvent::MouseInput { state, button, .. } => {
                self.mouse_actions.push((*button, *state));

                match state {
                    ElementState::Pressed => self.mouse_down.push(button.to_owned()),
                    ElementState::Released => {
                        let idx = self.mouse_down.iter().position(|x| x == button);
                        if let Some(idx) = idx {
                            self.mouse_down.remove(idx);
                        }
                    }
                }
            }
            WindowEvent::MouseWheel { delta, .. } => {
                self.scroll_delta += match delta {
                    MouseScrollDelta::LineDelta(_, y) => *y,
                    MouseScrollDelta::PixelDelta(pos) => pos.y as f32 / 10.0,
                };
            }
            WindowEvent::KeyboardInput { event, .. } if !event.repeat => {
                self.key_actions.push(event.clone());

                match event.state {
                    ElementState::Pressed => self.key_down.push(event.physical_key),
                    ElementState::Released => {
                        let idx = self.key_down.iter().position(|x| x == &event.physical_key);
                        if let Some(idx) = idx {
                            self.key_down.remove(idx);
                        }
                    }
                }
            }
            _ => {}
        }
    }

    pub(crate) fn on_frame_end(&mut self) {
        self.mouse_actions.clear();
        self.key_actions.clear();
        self.mouse = self.mouse.map(|x| x.abs());
        self.mouse_delta = Vector2::new(0.0, 0.0);
        self.scroll_delta = 0.0;
        self.resized = None;
        self.dpi_changed = None;
        self.clicks_canceled = false;
    }
}
