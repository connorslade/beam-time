use std::{cell::RefCell, f32::consts::PI, mem};

use bitflags::bitflags;
use engine::{
    assets::SpriteRef,
    color::Rgb,
    drawable::sprite::Sprite,
    exports::{
        nalgebra::Vector2,
        winit::{event::MouseButton, window::CursorIcon},
    },
    graphics_context::{Anchor, Drawable, GraphicsContext},
    layout::{LayoutElement, bounds::Bounds2D, tracker::LayoutTracker},
    memory::MemoryKey,
};

use crate::{
    assets::{BUTTON_CLICK, BUTTON_HOVER, LEVEL_DROPDOWN_ARROW},
    consts::ACCENT_COLOR,
};

pub struct Button {
    asset: SpriteRef,
    key: MemoryKey,
    effects: ButtonEffects,

    color: Rgb<f32>,
    pos: Vector2<f32>,
    anchor: Anchor,
    scale: Vector2<f32>,

    sprite: RefCell<Option<Sprite>>,
}

bitflags! {
    pub struct ButtonEffects: u8 {
        const Scale = 1;
        const Color = 2;
        const Arrows = 4;
    }
}

#[derive(Default)]
struct ButtonState {
    hover_time: f32,
    last_hovered: bool,
}

impl Button {
    pub fn new(asset: SpriteRef, key: MemoryKey) -> Self {
        Self {
            asset,
            key,
            effects: ButtonEffects::Scale,

            color: Rgb::new(1.0, 1.0, 1.0),
            pos: Vector2::zeros(),
            anchor: Anchor::BottomLeft,
            scale: Vector2::repeat(1.0),

            sprite: RefCell::new(None),
        }
    }

    pub fn aesthetic(self, aesthetic: ButtonEffects) -> Self {
        Self {
            effects: aesthetic,
            ..self
        }
    }

    pub fn pos(mut self, pos: Vector2<f32>, anchor: Anchor) -> Self {
        self.invalidate_sprite();
        self.pos = pos;
        self.anchor = anchor;
        self
    }

    pub fn scale(mut self, scale: Vector2<f32>) -> Self {
        self.invalidate_sprite();
        self.scale = scale;
        self
    }

    pub fn is_clicked(&self, ctx: &mut GraphicsContext) -> bool {
        let hovered = LayoutTracker::new(self.key).hovered(ctx);
        hovered && ctx.input.mouse_pressed(MouseButton::Left)
    }
}

impl Button {
    fn invalidate_sprite(&self) {
        self.sprite.replace(None);
    }

    fn generate_sprite(&self) {
        if self.sprite.borrow().is_some() {
            return;
        }

        let sprite = Sprite::new(self.asset)
            .position(Vector2::zeros(), self.anchor)
            .scale(self.scale);
        self.sprite.replace(Some(sprite));
    }
}

impl Drawable for Button {
    fn draw(self, ctx: &mut GraphicsContext) {
        let tracker = LayoutTracker::new(self.key);
        let hover = tracker.hovered(ctx);
        if hover {
            ctx.set_cursor(CursorIcon::Pointer);
        }

        let state = ctx.memory.get_or_insert(self.key, ButtonState::default());
        state.hover_time += ctx.delta_time * if hover { 1.0 } else { -1.0 };
        state.hover_time = state.hover_time.clamp(0.0, 0.1);
        let t = state.hover_time / 0.1;

        if hover && !mem::replace(&mut state.last_hovered, hover) {
            ctx.audio.builder(BUTTON_HOVER).with_gain(0.2).play_now();
        }

        if hover && ctx.input.mouse_pressed(MouseButton::Left) {
            ctx.audio.builder(BUTTON_CLICK).play_now();
        }

        self.generate_sprite();
        let mut sprite = self.sprite.take().unwrap();
        sprite = sprite.position(self.pos, self.anchor);

        if self.effects.contains(ButtonEffects::Scale) {
            let scale = self.scale + Vector2::repeat(t / 20.0).component_mul(&self.scale);
            sprite = sprite.dynamic_scale(scale, Anchor::Center);
        }

        if self.effects.contains(ButtonEffects::Color) {
            sprite = sprite.color(self.color.lerp(ACCENT_COLOR, t));
        }

        sprite.tracked(tracker).draw(ctx);

        if self.effects.contains(ButtonEffects::Arrows) && hover {
            let size = ctx.assets.get_sprite(self.asset).size;
            let y_offset =
                Vector2::y() * (size.y as f32 / 2.0 - 3.0) * self.scale.y * ctx.scale_factor;
            let x_offset = Vector2::x() * size.x as f32 * self.scale.x * ctx.scale_factor;
            let padding = Vector2::x() * (3.0 + t * 2.0) * self.scale.x * ctx.scale_factor;
            Sprite::new(LEVEL_DROPDOWN_ARROW)
                .scale(self.scale)
                .position(self.pos + y_offset - padding, self.anchor)
                .draw(ctx);
            Sprite::new(LEVEL_DROPDOWN_ARROW)
                .scale(self.scale)
                .rotate(PI, Anchor::CenterLeft)
                .position(self.pos + y_offset + x_offset + padding, self.anchor)
                .draw(ctx);
        }
    }
}

impl LayoutElement for Button {
    fn translate(&mut self, distance: Vector2<f32>) {
        self.pos += distance;
    }

    fn bounds(&self, ctx: &mut GraphicsContext) -> Bounds2D {
        self.generate_sprite();
        let sprite = self.sprite.borrow();
        let sprite = sprite.as_ref().unwrap();

        sprite.bounds(ctx).translated(self.pos)
    }

    fn draw(self: Box<Self>, ctx: &mut GraphicsContext) {
        (*self).draw(ctx);
    }
}
