use nalgebra::Vector2;

use crate::graphics_context::GraphicsContext;
#[cfg(feature = "layout_debug")]
use crate::{color::Rgb, graphics_context::Drawable};

use super::{bounds::Bounds2D, LayoutElement};

#[derive(Default)]
pub struct Container {
    bounds: Bounds2D,
    children: Vec<Box<dyn LayoutElement>>,
}

impl Container {
    pub fn insert(&mut self, bounds: Bounds2D, element: impl LayoutElement + 'static) {
        self.children.push(Box::new(element));
        self.bounds += bounds;
    }

    pub fn draw(&self, ctx: &mut GraphicsContext) {
        #[cfg(feature = "layout_debug")]
        {
            let outline = self.bounds.outline();
            outline.color(Rgb::hex(0x00FFFF)).draw(ctx)
        }

        for child in &self.children {
            #[cfg(feature = "layout_debug")]
            {
                let outline = child.bounds(ctx).outline();
                outline.color(Rgb::hex(0xFF0000)).draw(ctx);
            }

            child.draw(ctx);
        }
    }
}

impl LayoutElement for Container {
    fn translate(&mut self, distance: Vector2<f32>) {
        self.bounds.translate(distance);
        for child in &mut self.children {
            child.translate(distance);
        }
    }

    fn bounds(&self, _ctx: &mut GraphicsContext) -> Bounds2D {
        self.bounds
    }

    fn draw(&self, ctx: &mut GraphicsContext) {
        Container::draw(self, ctx);
    }
}
