use std::mem;

use nalgebra::Vector2;
use winit::{
    event::{ElementState, KeyEvent, MouseButton, MouseScrollDelta, WindowEvent},
    keyboard::{KeyCode, PhysicalKey},
};

use crate::application::window::WindowManager;

#[derive(Default)]
pub struct InputManager {
    mouse: Vector2<f32>,
    mouse_delta: Vector2<f32>,
    scroll_delta: f32,

    clicks_canceled: bool,
    mouse_down: Vec<MouseButton>,
    mouse_actions: Vec<(MouseButton, ElementState)>,

    key_down: Vec<PhysicalKey>,
    key_actions: Vec<KeyEvent>,
}

impl InputManager {
    pub(crate) fn on_window_event(&mut self, window_event: &WindowEvent, window: &WindowManager) {
        match window_event {
            WindowEvent::CursorMoved { position: pos, .. } => {
                let scale = window.scale_factor();
                let new_mouse =
                    Vector2::new(pos.x as f32 / scale, window.size.y - pos.y as f32 / scale);
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
        self.clicks_canceled = false;
    }
}

impl InputManager {
    #[inline(always)]
    pub fn mouse(&self) -> Vector2<f32> {
        self.mouse
    }

    #[inline(always)]
    pub fn mouse_delta(&self) -> Vector2<f32> {
        self.mouse_delta
    }

    #[inline(always)]
    pub fn scroll_delta(&self) -> f32 {
        self.scroll_delta
    }

    #[inline(always)]
    pub fn mouse_down(&self, button: MouseButton) -> bool {
        !self.clicks_canceled && self.mouse_down.contains(&button)
    }

    pub fn mouse_pressed(&self, button: MouseButton) -> bool {
        self.mouse_actions
            .iter()
            .any(|(b, s)| b == &button && s == &ElementState::Pressed)
    }

    pub fn consume_mouse_pressed(&mut self, button: MouseButton) -> bool {
        self.mouse_actions
            .iter()
            .position(|(b, s)| b == &button && s == &ElementState::Pressed)
            .inspect(|idx| {
                self.mouse_actions.remove(*idx);
            })
            .is_some()
    }

    pub fn mouse_released(&self, button: MouseButton) -> bool {
        self.mouse_actions
            .iter()
            .any(|(b, s)| b == &button && s == &ElementState::Released)
    }

    #[inline(always)]
    pub fn cancel_clicks(&mut self) {
        self.mouse_actions.clear();
        self.clicks_canceled = true;
    }

    #[inline(always)]
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

    pub fn consume_key_actions(&mut self) -> Vec<KeyEvent> {
        mem::take(&mut self.key_actions)
    }
}
