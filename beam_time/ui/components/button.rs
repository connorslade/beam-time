use bitflags::bitflags;
use engine::{
    color::Rgb,
    drawable::{sprite::Sprite, text::Text},
    exports::{
        nalgebra::Vector2,
        winit::{event::MouseButton, window::CursorIcon},
    },
    graphics_context::{Anchor, Drawable, GraphicsContext},
    layout::{LayoutElement, bounds::Bounds2D, tracker::LayoutTracker},
    memory::MemoryKey,
};

use crate::{
    assets::{BUTTON_CLICK, BUTTON_HOVER, LEFT_ARROW, RIGHT_ARROW},
    consts::ACCENT_COLOR,
};

pub struct Button<T: ButtonContent> {
    asset: T,
    key: MemoryKey,
    effects: ButtonEffects,
}

#[derive(Default)]
struct ButtonState {
    hover_time: f32,
    last_hovered: bool,
}

bitflags! {
    pub struct ButtonEffects: u8 {
        const Scale = 1;
        const Color = 2;
        const Arrows = 4;
    }
}

pub trait ButtonContent: Drawable + LayoutElement {
    fn position(self, position: Vector2<f32>, anchor: Anchor) -> Self;
    fn dynamic_scale(self, scale: Vector2<f32>, anchor: Anchor) -> Self;
    fn color(self, color: impl Into<Rgb<f32>>) -> Self;

    fn get_scale(&self) -> Vector2<f32>;
    fn get_color(&self) -> Rgb<f32>;
}

pub trait ButtonExt
where
    Self: ButtonContent + Sized,
{
    fn button(self, key: MemoryKey) -> Button<Self>;
}

impl<T: ButtonContent> Button<T> {
    pub fn new(key: MemoryKey, asset: T) -> Self {
        Self {
            asset,
            key,
            effects: ButtonEffects::Scale,
        }
    }

    pub fn effects(self, effects: ButtonEffects) -> Self {
        Self { effects, ..self }
    }

    pub fn is_clicked(&self, ctx: &mut GraphicsContext) -> bool {
        let hovered = LayoutTracker::new(self.key).hovered(ctx);
        hovered && ctx.input.mouse_pressed(MouseButton::Left)
    }
}

impl<T: ButtonContent + 'static> Drawable for Button<T> {
    fn draw(mut self, ctx: &mut GraphicsContext) {
        let tracker = LayoutTracker::new(self.key);
        let hover = tracker.hovered(ctx);
        hover.then(|| ctx.set_cursor(CursorIcon::Pointer));

        let state = ctx.memory.get_or_insert(self.key, ButtonState::default());
        state.hover_time += ctx.delta_time * if hover { 1.0 } else { -1.0 };
        state.hover_time = state.hover_time.clamp(0.0, 0.1);
        let t = state.hover_time / 0.1;

        if hover && !state.last_hovered {
            ctx.audio.builder(BUTTON_HOVER).with_gain(0.2).play_now();
        }
        state.last_hovered = hover;

        if hover && ctx.input.mouse_pressed(MouseButton::Left) {
            ctx.audio.builder(BUTTON_CLICK).play_now();
        }

        let (scale, color) = (self.asset.get_scale(), self.asset.get_color());
        if self.effects.contains(ButtonEffects::Scale) {
            let scale = scale + Vector2::repeat(t / 20.0).component_mul(&scale);
            self.asset = self.asset.dynamic_scale(scale, Anchor::Center);
        }

        if self.effects.contains(ButtonEffects::Color) {
            self.asset = self.asset.color(color.lerp(ACCENT_COLOR, t));
        }

        let bounds = self.asset.bounds(ctx);
        if self.effects.contains(ButtonEffects::Arrows) && hover {
            let px = scale * ctx.scale_factor;
            let padding = Vector2::x() * t * 2.0 * px.x;
            let base = Vector2::y() * ((bounds.min.y + bounds.max.y) / 2.0 - px.y);
            let (x_min, x_max) = (Vector2::x() * bounds.min.x, Vector2::x() * bounds.max.x);
            Sprite::new(RIGHT_ARROW)
                .scale(scale)
                .position(base - padding + x_min, Anchor::CenterRight)
                .draw(ctx);
            Sprite::new(LEFT_ARROW)
                .scale(scale)
                .position(base + padding + x_max, Anchor::CenterLeft)
                .draw(ctx);
        }

        self.asset.tracked(tracker).draw(ctx);
    }
}

impl<T: ButtonContent + 'static> LayoutElement for Button<T> {
    fn translate(&mut self, distance: Vector2<f32>) {
        self.asset.translate(distance);
    }

    fn bounds(&self, ctx: &mut GraphicsContext) -> Bounds2D {
        self.asset.bounds(ctx)
    }

    fn draw(self: Box<Self>, ctx: &mut GraphicsContext) {
        (*self).draw(ctx);
    }
}

impl ButtonContent for Sprite {
    fn position(self, position: Vector2<f32>, anchor: Anchor) -> Self {
        self.position(position, anchor)
    }

    fn dynamic_scale(self, scale: Vector2<f32>, anchor: Anchor) -> Self {
        self.dynamic_scale(scale, anchor)
    }

    fn color(self, color: impl Into<Rgb<f32>>) -> Self {
        self.color(color)
    }

    fn get_scale(&self) -> Vector2<f32> {
        self.get_scale()
    }

    fn get_color(&self) -> Rgb<f32> {
        self.get_color()
    }
}

impl ButtonContent for Text {
    fn position(self, position: Vector2<f32>, anchor: Anchor) -> Self {
        self.position(position, anchor)
    }

    fn dynamic_scale(self, scale: Vector2<f32>, anchor: Anchor) -> Self {
        self.dynamic_scale(scale, anchor)
    }

    fn color(self, color: impl Into<Rgb<f32>>) -> Self {
        self.color(color)
    }

    fn get_scale(&self) -> Vector2<f32> {
        self.get_scale()
    }

    fn get_color(&self) -> Rgb<f32> {
        self.get_color()
    }
}

impl<T: ButtonContent + Sized> ButtonExt for T {
    fn button(self, key: MemoryKey) -> Button<Self> {
        Button::new(key, self)
    }
}
