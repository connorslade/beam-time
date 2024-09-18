use crate::graphics_context::GraphicsContext;

pub trait Screen {
    fn render(&mut self, ctx: &mut GraphicsContext);
    fn update(&mut self, _ctx: &mut GraphicsContext) {}
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

    pub fn pop_n(&mut self, n: usize) {
        for _ in 0..n {
            self.inner.pop();
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
}
