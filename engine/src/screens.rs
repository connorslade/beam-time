use nalgebra::Vector2;

use crate::graphics_context::GraphicsContext;

pub trait Screen {
    fn render(&mut self, ctx: &mut GraphicsContext);
    fn update(&mut self, _ctx: &mut GraphicsContext) {}

    fn on_init(&mut self) {}
    fn on_resize(&mut self, _size: Vector2<f32>) {}
}

pub struct Screens {
    inner: Vec<Box<dyn Screen>>,
}

impl Screens {
    pub fn new(inner: Vec<Box<dyn Screen>>) -> Self {
        Self { inner }
    }

    pub fn pop_n(&mut self, n: usize) {
        for _ in 0..n {
            self.inner.pop();
        }

        if n > 0 {
            self.top().on_init();
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
        self.inner.iter_mut().for_each(|x| x.update(ctx));
    }

    pub fn on_resize(&mut self, size: Vector2<f32>) {
        self.top().on_resize(size);
        self.inner.iter_mut().for_each(|x| x.on_resize(size));
    }
}
