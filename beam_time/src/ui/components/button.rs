use std::cell::RefCell;

use engine::{
    assets::SpriteRef,
    color::Rgb,
    drawable::sprite::Sprite,
    exports::{nalgebra::Vector2, winit::event::MouseButton},
    graphics_context::{Anchor, Drawable, GraphicsContext},
    layout::{bounds::Bounds2D, tracker::LayoutTracker, LayoutElement},
    memory::MemoryKey,
};

use crate::{
    assets::{BUTTON_CLICK, BUTTON_HOVER},
    consts::ACCENT_COLOR,
};

pub struct Button {
    asset: SpriteRef,
    key: MemoryKey,

    color: Rgb<f32>,
    pos: Vector2<f32>,
    anchor: Anchor,
    scale: Vector2<f32>,

    sprite: RefCell<Option<Sprite>>,
}

#[derive(Default)]
pub struct ButtonState {
    hover_time: f32,
    last_hovered: bool,
}

impl Button {
    pub fn new(asset: SpriteRef, key: MemoryKey) -> Self {
        Self {
            asset,
            key,

            color: Rgb::new(1.0, 1.0, 1.0),
            pos: Vector2::zeros(),
            anchor: Anchor::BottomLeft,
            scale: Vector2::repeat(1.0),

            sprite: RefCell::new(None),
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

        let state = ctx.memory.get_or_insert(self.key, ButtonState::default());
        state.hover_time += ctx.delta_time * if hover { 1.0 } else { -1.0 };
        state.hover_time = state.hover_time.clamp(0.0, 0.1);

        let color = self.color.lerp(ACCENT_COLOR, state.hover_time / 0.1);
        let scale = self.scale + Vector2::repeat(state.hover_time / 2.0).component_mul(&self.scale);

        self.generate_sprite();
        let sprite = self.sprite.take().unwrap();
        let sprite = sprite
            .position(self.pos, self.anchor)
            .color(color)
            .dynamic_scale(scale, Anchor::Center)
            .tracked(tracker);

        if hover && !state.last_hovered {
            ctx.audio.builder(BUTTON_HOVER).with_gain(0.2).play_now();
        }

        if hover && ctx.input.mouse_pressed(MouseButton::Left) {
            ctx.audio.builder(BUTTON_CLICK).play_now();
        }

        state.last_hovered = hover;
        sprite.draw(ctx);
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
