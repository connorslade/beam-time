use engine::graphics_context::{Drawable, GraphicsContext};

pub struct Modal {
    layer: i16,
}

impl Modal {}

impl<App> Drawable<App> for Modal {
    fn draw(self, ctx: &mut GraphicsContext<App>) {
        todo!()
    }
}
