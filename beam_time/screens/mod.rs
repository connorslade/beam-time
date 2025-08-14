use std::mem;

use engine::graphics_context::GraphicsContext;

use crate::app::App;

pub mod campaign;
pub mod debug_overlay;
pub mod game;
pub mod sandbox;
pub mod title;

pub trait Screen {
    fn tick(&mut self, _state: &mut App, _ctx: &mut GraphicsContext) {}
    fn render(&mut self, _state: &mut App, _ctx: &mut GraphicsContext) {}
    fn post_render(&mut self, _state: &mut App, _ctx: &mut GraphicsContext) {}

    fn on_init(&mut self, _state: &mut App) {}
    fn on_destroy(&mut self, _state: &mut App) {}
}

pub struct Screens {
    inner: Vec<Box<dyn Screen>>,
    new_screen: bool,
}

impl Screens {
    pub fn new(inner: Vec<Box<dyn Screen>>) -> Self {
        Self {
            inner,
            new_screen: false,
        }
    }

    pub fn extend(&mut self, mut screens: Vec<Box<dyn Screen>>, state: &mut App) {
        screens.iter_mut().for_each(|s| s.on_init(state));
        self.new_screen |= !screens.is_empty();
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
            self.new_screen = true;
        }
    }

    pub fn destroy(&mut self, state: &mut App) {
        while let Some(mut screen) = self.inner.pop() {
            screen.on_destroy(state);
        }
    }

    pub fn top(&mut self) -> Option<&mut Box<dyn Screen>> {
        self.inner.last_mut()
    }

    pub fn render(&mut self, ctx: &mut GraphicsContext, state: &mut App) {
        mem::take(&mut self.new_screen).then(|| ctx.input.cancel_clicks());

        self.inner.iter_mut().for_each(|x| x.tick(state, ctx));
        if let Some(top) = self.top() {
            top.render(state, ctx);
        }
        self.inner
            .iter_mut()
            .for_each(|x| x.post_render(state, ctx));
    }
}
