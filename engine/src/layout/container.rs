use nalgebra::Vector2;

use crate::graphics_context::GraphicsContext;
#[cfg(feature = "layout_debug")]
use crate::{color::Rgb, graphics_context::Drawable};

use super::{bounds::Bounds2D, LayoutElement, SizedLayoutElement};

#[derive(Default)]
pub struct Container {
    pub(crate) bounds: Bounds2D,
    pub(crate) children: Vec<SizedLayoutElement>,
}

impl Container {
    pub fn of(
        ctx: &mut GraphicsContext,
        elements: impl IntoIterator<Item = Box<dyn LayoutElement>>,
    ) -> Self {
        let mut container = Self::default();
        for element in elements.into_iter() {
            container.insert(SizedLayoutElement::new(ctx, element));
        }

        container
    }

    pub fn insert(&mut self, element: SizedLayoutElement) {
        self.bounds += element.bounds;
        self.children.push(element);
    }

    pub fn draw(self, ctx: &mut GraphicsContext) {
        #[cfg(feature = "layout_debug")]
        {
            let outline = self.bounds.outline();
            outline.color(Rgb::hex(0x00FFFF)).draw(ctx)
        }

        for child in self.children {
            #[cfg(feature = "layout_debug")]
            {
                let outline = child.bounds(ctx).outline();
                outline.color(Rgb::hex(0xFF0000)).draw(ctx);
            }

            child.element.draw(ctx);
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

    fn draw(self: Box<Self>, ctx: &mut GraphicsContext) {
        Container::draw(*self, ctx);
    }
}
