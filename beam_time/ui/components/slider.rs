use common::misc::in_bounds;
use engine::{
    drawable::{
        Anchor, Drawable, shape::rectangle::Rectangle, spacer::Spacer, sprite::Sprite, text::Text,
    },
    exports::{
        nalgebra::Vector2,
        winit::{event::MouseButton, window::CursorIcon},
    },
    graphics_context::GraphicsContext,
    layout::{Justify, Layout, LayoutElement, LayoutMethods, bounds::Bounds2D, row::RowLayout},
    memory::{Memory, MemoryKey},
    memory_key,
};

use crate::{
    assets::{SLIDER_HANDLE, UNDEAD_FONT},
    consts::color,
};

pub struct Slider {
    position: Vector2<f32>,
    width: f32,
    bounds: (f32, f32),
    start: f32,
    default: Option<f32>,
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
            start: 0.0,
            default: None,
            key,
        }
    }

    pub fn start(self, start: f32) -> Self {
        Self { start, ..self }
    }

    pub fn default(self, default: f32) -> Self {
        Self {
            default: Some(default),
            ..self
        }
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
            .unwrap_or(self.start)
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
            t: ((self.start - min) / (max - min)).clamp(0.0, 1.0),
            ..Default::default()
        })
    }
}

impl Drawable for Slider {
    fn draw(self, ctx: &mut GraphicsContext) {
        let px = 4.0;
        let width = self.width - px * 4.0;

        let state = self.state(ctx.memory);

        let mouse = ctx.input.mouse();
        if state.dragging {
            let dx = mouse.x - self.position.x - state.offset;
            state.t = (dx / width).clamp(0.0, 1.0);
            state.dragging = ctx.input.mouse_down(MouseButton::Left);
        }

        let offset = Vector2::x() * (state.t * width);
        let handle = Sprite::new(SLIDER_HANDLE)
            .position(self.position + offset, Anchor::BottomLeft)
            .scale(Vector2::repeat(4.0))
            .z_index(1);

        let click = ctx.input.mouse_pressed(MouseButton::Left);
        let right = ctx.input.mouse_pressed(MouseButton::Right);

        let hovered = handle.is_hovered(ctx);
        hovered.then(|| ctx.window.cursor(CursorIcon::Pointer));

        let state = self.state(ctx.memory);
        let size = Vector2::new(self.width, px * 6.0);
        let in_bounds = in_bounds(mouse, (self.position, self.position + size));
        if click && hovered {
            state.offset = mouse.x - (self.position.x + offset.x);
            state.dragging = true;
        } else if click && in_bounds {
            state.offset = px * 2.0;
            state.dragging = true;
        }

        if right
            && !state.dragging
            && in_bounds
            && let Some(default) = self.default
        {
            let (min, max) = self.bounds;
            state.t = (default - min) / (max - min);
        }

        Rectangle::new(Vector2::new(self.width, px))
            .position(self.position + Vector2::y() * px * 2.5, Anchor::BottomLeft)
            .color(color::MODAL_BORDER)
            .draw(ctx);
        handle.draw(ctx);
    }
}

impl LayoutElement for Slider {
    fn translate(&mut self, distance: Vector2<f32>) {
        self.position += distance;
    }

    fn bounds(&self, _ctx: &mut GraphicsContext) -> Bounds2D {
        let size = Vector2::new(self.width, 6.0 * 4.0);
        Bounds2D::new(self.position, self.position + size)
    }

    fn draw(self: Box<Self>, ctx: &mut GraphicsContext) {
        (*self).draw(ctx);
    }
}

pub fn slider<L: Layout + LayoutElement + 'static>(
    (ctx, layout): (&mut GraphicsContext, &mut L),
    name: &str,
    value: &mut f32,
    (min, default, max): (f32, f32, f32),
) {
    Text::new(UNDEAD_FONT, name)
        .scale(Vector2::repeat(2.0))
        .layout(ctx, layout);
    layout.nest(
        ctx,
        RowLayout::new(10.0).justify(Justify::Center),
        |ctx, layout| {
            let slider = Slider::new(memory_key!(name))
                .start(*value)
                .default(default)
                .bounds(min, max);
            *value = slider.value(ctx);
            slider.layout(ctx, layout);

            let max_digits = (max.round() as u32).ilog10() + 4;
            let max_width = max_digits as f32 * 4.0 * 2.0;

            let text = Text::new(UNDEAD_FONT, format!("{value:.2}")).scale(Vector2::repeat(2.0));
            let space = max_width - text.size(ctx).x;

            text.layout(ctx, layout);
            Spacer::new_x(space).layout(ctx, layout);
        },
    );
}
