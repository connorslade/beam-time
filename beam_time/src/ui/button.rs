use engine::{
    assets::SpriteRef,
    color::Rgb,
    drawable::sprite::Sprite,
    exports::{nalgebra::Vector2, winit::event::MouseButton},
    graphics_context::{Anchor, Drawable, GraphicsContext},
    layout::{tracker::LayoutTracker, LayoutElement},
    memory::MemoryKey,
};

use crate::{
    assets::{BUTTON_BACK, BUTTON_HOVER, BUTTON_SUCCESS},
    consts::ACCENT_COLOR,
};

pub struct Button {
    asset: SpriteRef,
    key: MemoryKey,
    is_back: bool,

    color: Rgb<f32>,
    pos: Vector2<f32>,
    anchor: Anchor,
    scale: Vector2<f32>,
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
            is_back: false,

            color: Rgb::new(1.0, 1.0, 1.0),
            pos: Vector2::zeros(),
            anchor: Anchor::BottomLeft,
            scale: Vector2::repeat(1.0),
        }
    }

    pub fn pos(mut self, pos: Vector2<f32>, anchor: Anchor) -> Self {
        self.pos = pos;
        self.anchor = anchor;
        self
    }

    pub fn scale(mut self, scale: Vector2<f32>) -> Self {
        self.scale = scale;
        self
    }

    pub fn set_back(mut self) -> Self {
        self.is_back = true;
        self
    }

    pub fn is_clicked(&self, ctx: &mut GraphicsContext) -> bool {
        let hovered = LayoutTracker::new(self.key).hovered(ctx);
        hovered && ctx.input.mouse_pressed(MouseButton::Left)
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

        let sprite = Sprite::new(self.asset)
            .color(color)
            .position(self.pos, self.anchor)
            .scale(scale)
            .tracked(tracker);

        if hover && !state.last_hovered {
            ctx.audio.builder(BUTTON_HOVER).with_gain(0.2).play_now();
        }

        if hover && ctx.input.mouse_pressed(MouseButton::Left) {
            let sound = if self.is_back {
                BUTTON_BACK
            } else {
                BUTTON_SUCCESS
            };
            ctx.audio.builder(sound).play_now();
        }

        state.last_hovered = hover;
        sprite.draw(ctx);
    }
}
