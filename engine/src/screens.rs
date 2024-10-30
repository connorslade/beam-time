use nalgebra::Vector2;

use crate::graphics_context::GraphicsContext;

pub trait Screen<App> {
    fn render(&mut self, _state: &mut App, _ctx: &mut GraphicsContext<App>) {}
    fn pre_render(&mut self, _state: &mut App, _ctx: &mut GraphicsContext<App>) {}
    fn post_render(&mut self, _state: &mut App, _ctx: &mut GraphicsContext<App>) {}

    fn on_init(&mut self, _state: &mut App) {}
    fn on_resize(&mut self, _state: &mut App, _old_size: Vector2<f32>, _new_size: Vector2<f32>) {}
    fn on_destroy(&mut self, _state: &mut App) {}
}

pub struct Screens<App> {
    inner: Vec<Box<dyn Screen<App>>>,
}

impl<App> Screens<App> {
    pub fn new(inner: Vec<Box<dyn Screen<App>>>) -> Self {
        Self { inner }
    }

    pub fn pop_n(&mut self, n: usize, state: &mut App) {
        for _ in 0..n {
            if let Some(mut screen) = self.inner.pop() {
                screen.on_destroy(state);
            }
        }

        if n > 0 {
            self.top().on_init(state);
        }
    }

    pub fn extend(&mut self, mut screens: Vec<Box<dyn Screen<App>>>, state: &mut App) {
        screens.iter_mut().for_each(|x| x.on_init(state));
        self.inner.extend(screens);
    }

    fn top(&mut self) -> &mut Box<dyn Screen<App>> {
        self.inner.last_mut().unwrap()
    }

    pub fn render(&mut self, ctx: &mut GraphicsContext<App>, state: &mut App) {
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

    pub fn destroy(&mut self, state: &mut App) {
        self.inner.iter_mut().for_each(|x| x.on_destroy(state));
    }
}
