use common::misc::in_bounds;
use engine::{
    drawable::{shape::rectangle::Rectangle, sprite::Sprite},
    exports::{nalgebra::Vector2, winit::event::MouseButton},
    graphics_context::{Anchor, Drawable, GraphicsContext},
    layout::{LayoutElement, bounds::Bounds2D},
    memory::{Memory, MemoryKey},
};

use crate::{assets::SLIDER_HANDLE, consts::MODAL_BORDER_COLOR};

pub struct Slider {
    position: Vector2<f32>,
    width: f32,
    bounds: (f32, f32),
    default: f32,
    key: MemoryKey,
}

#[derive(Default)]
struct SliderState {
    t: f32,
    offset: f32,
    dragging: bool,
}

impl Slider {
    pub fn new(key: MemoryKey) -> Self {
        Self {
            position: Vector2::zeros(),
            width: 150.0,
            bounds: (0.0, 1.0),
            default: 0.0,
            key,
        }
    }

    pub fn default(self, default: f32) -> Self {
        Self { default, ..self }
    }

    pub fn width(self, width: f32) -> Self {
        Self { width, ..self }
    }

    pub fn bounds(self, min: f32, max: f32) -> Self {
        Self {
            bounds: (min, max),
            ..self
        }
    }

    pub fn value(&self, ctx: &GraphicsContext) -> f32 {
        let (min, max) = self.bounds;
        ctx.memory
            .get::<SliderState>(self.key)
            .map(|x| x.t * (max - min) + min)
            .unwrap_or(min)
    }

    pub fn is_dragging(&self, ctx: &GraphicsContext) -> bool {
        ctx.memory
            .get::<SliderState>(self.key)
            .map(|x| x.dragging)
            .unwrap_or_default()
    }
}

impl Slider {
    fn state<'a>(&self, memory: &'a mut Memory) -> &'a mut SliderState {
        let (min, max) = self.bounds;
        memory.get_or_insert_with(self.key, || SliderState {
            t: ((self.default - min) / (max - min)).clamp(0.0, 1.0),
            ..Default::default()
        })
    }
}

impl Drawable for Slider {
    fn draw(self, ctx: &mut GraphicsContext) {
        let px = 4.0 * ctx.scale_factor;
        let full_width = self.width * ctx.scale_factor;
        let width = full_width - px * 4.0;

        let state = self.state(ctx.memory);
        let offset = Vector2::x() * (state.t * width);

        if state.dragging {
            let dx = ctx.input.mouse.x - self.position.x - state.offset;
            state.t = (dx / width).clamp(0.0, 1.0);
            state.dragging = ctx.input.mouse_down(MouseButton::Left);
        }

        let handle = Sprite::new(SLIDER_HANDLE)
            .position(self.position + offset, Anchor::BottomLeft)
            .scale(Vector2::repeat(4.0))
            .z_index(1);

        let click = ctx.input.mouse_pressed(MouseButton::Left);
        let hovered = handle.is_hovered(ctx);

        let state = self.state(ctx.memory);
        let size = Vector2::new(full_width, px * 6.0);
        if click && hovered {
            state.dragging = true;
            state.offset = ctx.input.mouse.x - (self.position.x + offset.x);
        } else if click && in_bounds(ctx.input.mouse, (self.position, self.position + size)) {
            state.dragging = true;
            state.offset = px * 2.0;
        }

        Rectangle::new(Vector2::new(full_width, px))
            .position(self.position + Vector2::y() * px * 2.5, Anchor::BottomLeft)
            .color(MODAL_BORDER_COLOR)
            .draw(ctx);
        handle.draw(ctx);
    }
}

impl LayoutElement for Slider {
    fn translate(&mut self, distance: Vector2<f32>) {
        self.position += distance;
    }

    fn bounds(&self, ctx: &mut GraphicsContext) -> Bounds2D {
        let size = Vector2::new(self.width, 6.0 * 4.0) * ctx.scale_factor;
        Bounds2D::new(self.position, self.position + size)
    }

    fn draw(self: Box<Self>, ctx: &mut GraphicsContext) {
        (*self).draw(ctx);
    }
}
