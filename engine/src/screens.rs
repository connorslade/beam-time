use winit::event::{ElementState, KeyEvent, MouseButton};

use crate::graphics_context::GraphicsContext;

pub trait Screen {
    fn render(&mut self, ctx: &mut GraphicsContext);

    fn on_key(&mut self, _key_event: KeyEvent) {}
    fn on_click(&mut self, _state: ElementState, _button: MouseButton) {}
}

pub struct Screens {
    inner: Vec<Box<dyn Screen>>,
}

impl Screens {
    pub fn new(screen: Box<dyn Screen>) -> Self {
        Self {
            inner: vec![screen],
        }
    }

    pub fn extend(&mut self, screens: Vec<Box<dyn Screen>>) {
        self.inner.extend(screens);
    }

    fn top(&mut self) -> &mut Box<dyn Screen> {
        self.inner.last_mut().unwrap()
    }

    pub fn render(&mut self, ctx: &mut GraphicsContext) {
        self.top().render(ctx);
    }

    pub fn on_key(&mut self, key_event: KeyEvent) {
        self.top().on_key(key_event);
    }

    pub fn on_click(&mut self, state: ElementState, button: MouseButton) {
        self.top().on_click(state, button);
    }
}
