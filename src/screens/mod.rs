use crate::graphics_context::GraphicsContext;

mod title;

pub trait Screen {
    fn render(&mut self, ctx: &mut GraphicsContext);
}

pub struct Screens {
    inner: Vec<Box<dyn Screen>>,
}

impl Screens {
    pub fn render(&mut self, ctx: &mut GraphicsContext) {
        let top = self.inner.last_mut().unwrap();
        top.render(ctx);
    }
}

impl Default for Screens {
    fn default() -> Self {
        Self {
            inner: vec![Box::new(title::TitleScreen {})],
        }
    }
}
