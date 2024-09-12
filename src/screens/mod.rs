use wgpu::CommandEncoder;

mod title;

pub trait Screen {
    fn render(&mut self, encoder: &mut CommandEncoder);
}

pub struct Screens {
    inner: Vec<Box<dyn Screen>>,
}

impl Screens {
    pub fn render(&mut self, encoder: &mut CommandEncoder) {
        let top = self.inner.last_mut().unwrap();
        top.render(encoder);
    }
}

impl Default for Screens {
    fn default() -> Self {
        Self {
            inner: vec![Box::new(title::TitleScreen {})],
        }
    }
}
