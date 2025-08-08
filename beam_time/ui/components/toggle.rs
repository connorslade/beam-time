use engine::{
    drawable::{Anchor, Drawable},
    drawable::{sprite::Sprite, text::Text},
    exports::{
        nalgebra::Vector2,
        winit::{event::MouseButton, window::CursorIcon},
    },
    graphics_context::GraphicsContext,
    layout::{
        Justify, Layout, LayoutElement, LayoutMethods, bounds::Bounds2D, row::RowLayout,
        tracker::LayoutTracker,
    },
    memory_key,
};

use crate::assets::{TOGGLE_ACTIVE, TOGGLE_INACTIVE, UNDEAD_FONT};

struct Toggle {
    value: bool,

    position: Vector2<f32>,
    anchor: Anchor,
    scale: f32,
    z_index: i16,
}

impl Toggle {
    pub fn new(value: bool) -> Self {
        Self {
            value,
            position: Vector2::zeros(),
            anchor: Anchor::BottomLeft,
            scale: 4.0,
            z_index: 0,
        }
    }
}

impl Drawable for Toggle {
    fn draw(self, ctx: &mut GraphicsContext) {
        let sprite = [TOGGLE_INACTIVE, TOGGLE_ACTIVE][self.value as usize];
        Sprite::new(sprite)
            .position(self.position, self.anchor)
            .scale(Vector2::repeat(self.scale))
            .z_index(self.z_index)
            .draw(ctx);
    }
}

impl LayoutElement for Toggle {
    fn translate(&mut self, distance: Vector2<f32>) {
        self.position += distance;
    }

    fn bounds(&self, ctx: &mut GraphicsContext) -> Bounds2D {
        let size = Vector2::new(8.0, 5.0) * self.scale * ctx.scale_factor;
        Bounds2D::new(Vector2::zeros(), size).translated(self.position)
    }

    fn draw(self: Box<Self>, ctx: &mut GraphicsContext) {
        (*self).draw(ctx);
    }
}

pub fn toggle<L: Layout + LayoutElement + 'static>(
    ctx: &mut GraphicsContext,
    layout: &mut L,
    value: &mut bool,
    name: &str,
) {
    let tracker = LayoutTracker::new(memory_key!(name));
    let hover = tracker.hovered(ctx);
    hover.then(|| ctx.window.cursor(CursorIcon::Pointer));
    *value ^= hover && ctx.input.mouse_pressed(MouseButton::Left);

    layout.nest(
        ctx,
        RowLayout::new(10.0 * ctx.scale_factor)
            .justify(Justify::Center)
            .tracked(tracker),
        |ctx, layout| {
            Toggle::new(*value).layout(ctx, layout);
            Text::new(UNDEAD_FONT, name)
                .scale(Vector2::repeat(2.0))
                .layout(ctx, layout);
        },
    );
}
