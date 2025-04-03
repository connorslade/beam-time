use engine::{exports::nalgebra::Vector2, graphics_context::GraphicsContext};

use crate::app::App;

pub mod about;
pub mod campaign;
#[cfg(feature = "debug")]
pub mod debug_overlay;
pub mod game;
pub mod sandbox;
pub mod title;

pub trait Screen {
    fn render(&mut self, _state: &mut App, _ctx: &mut GraphicsContext) {}
    fn pre_render(&mut self, _state: &mut App, _ctx: &mut GraphicsContext) {}
    fn post_render(&mut self, _state: &mut App, _ctx: &mut GraphicsContext) {}

    fn on_init(&mut self, _state: &mut App) {}
    fn on_resize(&mut self, _state: &mut App, _old_size: Vector2<f32>, _new_size: Vector2<f32>) {}
    fn on_destroy(&mut self, _state: &mut App) {}
}

pub struct Screens {
    inner: Vec<Box<dyn Screen>>,
}

impl Screens {
    pub fn new(inner: Vec<Box<dyn Screen>>) -> Self {
        Self { inner }
    }

    pub fn extend(&mut self, mut screens: Vec<Box<dyn Screen>>, state: &mut App) {
        screens.iter_mut().for_each(|s| s.on_init(state));
        self.inner.extend(screens);
    }

    pub fn pop_n(&mut self, n: usize, state: &mut App) {
        for _ in 0..n {
            if let Some(mut screen) = self.inner.pop() {
                screen.on_destroy(state);
            }
        }

        if let (true, Some(screen)) = (n >= 1, self.inner.last_mut()) {
            screen.on_init(state);
        }
    }

    pub fn top(&mut self) -> &mut Box<dyn Screen> {
        self.inner.last_mut().unwrap()
    }

    pub fn render(&mut self, ctx: &mut GraphicsContext, state: &mut App) {
        self.inner.iter_mut().for_each(|x| x.pre_render(state, ctx));
        self.top().render(state, ctx);
        self.inner
            .iter_mut()
            .for_each(|x| x.post_render(state, ctx));
    }

    pub fn on_resize(&mut self, old_size: Vector2<f32>, new_size: Vector2<f32>, state: &mut App) {
        self.inner
            .iter_mut()
            .for_each(|x| x.on_resize(state, old_size, new_size));
    }
}
