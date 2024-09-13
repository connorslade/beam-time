use crate::graphics_context::GraphicsContext;

pub trait Screen {
    fn render(&mut self, ctx: &mut GraphicsContext);
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

    pub fn render(&mut self, ctx: &mut GraphicsContext) {
        let top = self.inner.last_mut().unwrap();
        top.render(ctx);
    }
}
