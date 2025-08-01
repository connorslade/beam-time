use nalgebra::Vector2;

use crate::{
    color::Rgb,
    drawable::RECTANGLE_POINTS,
    graphics_context::{Anchor, Drawable, GraphicsContext},
    layout::{LayoutElement, bounds::Bounds2D},
    render::shape::ShapeVertex,
};

#[derive(Clone)]
pub struct RectangleOutline {
    position: Vector2<f32>,
    position_anchor: Anchor,
    size: Vector2<f32>,
    z_index: i16,
    color: Rgb<f32>,
    thickness: f32,
}

impl RectangleOutline {
    pub fn new(size: Vector2<f32>, thickness: f32) -> Self {
        Self {
            position: Vector2::zeros(),
            position_anchor: Anchor::BottomLeft,
            size,
            z_index: 0,
            color: Rgb::repeat(1.0),
            thickness,
        }
    }

    pub fn thickness(mut self, thickness: f32) -> Self {
        self.thickness = thickness;
        self
    }

    pub fn position(mut self, position: Vector2<f32>, anchor: Anchor) -> Self {
        self.position = position;
        self.position_anchor = anchor;
        self
    }

    pub fn color(mut self, color: impl Into<Rgb<f32>>) -> Self {
        self.color = color.into();
        self
    }

    pub fn z_index(mut self, z_index: i16) -> Self {
        self.z_index = z_index;
        self
    }

    fn points(&self, ctx: &GraphicsContext) -> [[Vector2<f32>; 4]; 2] {
        let outer_size = self.size + Vector2::repeat(self.thickness * 2.0 * ctx.scale_factor);

        let offset_outer = self.position + self.position_anchor.offset(outer_size);
        let offset_inner = offset_outer + Vector2::repeat(self.thickness * ctx.scale_factor);

        [
            RECTANGLE_POINTS.map(|x| offset_outer + x.component_mul(&outer_size)),
            RECTANGLE_POINTS.map(|x| offset_inner + x.component_mul(&self.size)),
        ]
    }
}

impl Drawable for RectangleOutline {
    fn draw(self, ctx: &mut GraphicsContext) {
        let [outer, inner] = self.points(ctx).map(|y| {
            y.map(|x| ShapeVertex::new(x, self.color).z_index(self.z_index))
                .map(|x| ctx.shapes.push_vertex(x))
        });

        for i in 0..4 {
            let j = (i + 1) % 4;
            ctx.shapes.push_triangle(&[outer[i], outer[j], inner[i]]);
            ctx.shapes.push_triangle(&[inner[i], inner[j], outer[j]]);
        }
    }
}

impl LayoutElement for RectangleOutline {
    fn translate(&mut self, distance: Vector2<f32>) {
        self.position += distance;
    }

    fn bounds(&self, ctx: &mut GraphicsContext) -> Bounds2D {
        let outer_size = self.size + Vector2::repeat(self.thickness * 2.0 * ctx.scale_factor);
        let offset_outer = self.position + self.position_anchor.offset(outer_size);
        Bounds2D::new(offset_outer, offset_outer + outer_size)
    }

    fn draw(self: Box<Self>, ctx: &mut GraphicsContext) {
        (*self).draw(ctx);
    }
}
