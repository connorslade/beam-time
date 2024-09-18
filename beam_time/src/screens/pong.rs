use engine::{
    exports::winit::{
        event::KeyEvent,
        keyboard::{Key, NamedKey},
    },
    graphics_context::GraphicsContext,
    screens::Screen,
};

#[derive(Default)]
pub struct PongScreen {}

impl Screen for PongScreen {
    fn render(&mut self, ctx: &mut GraphicsContext) {}

    fn on_key(&mut self, key_event: KeyEvent) {
        if key_event.logical_key == Key::Named(NamedKey::Escape) {}
    }
}
