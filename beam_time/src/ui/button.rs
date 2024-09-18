use engine::{
    assets::AssetRef,
    color::Rgb,
    drawable::sprites::Sprite,
    exports::nalgebra::Vector2,
    graphics_context::{Anchor, Drawable, GraphicsContext},
};

use crate::consts::ACCENT_COLOR;

pub struct Button<'a> {
    asset: AssetRef,
    state: &'a mut ButtonState,

    pos: Vector2<f32>,
    anchor: Anchor,
    scale: Vector2<f32>,
}

#[derive(Default)]
pub struct ButtonState {
    hover_time: f32,
}

impl<'a> Button<'a> {
    pub fn new(asset: AssetRef, state: &'a mut ButtonState) -> Self {
        Self {
            asset,
            state,
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
}

impl<'a> Drawable for Button<'a> {
    fn draw(self, ctx: &mut GraphicsContext) {
        let sprite = Sprite::new(self.asset)
            .color(Rgb::new(1.0, 1.0, 1.0).lerp(ACCENT_COLOR, self.state.hover_time / 0.1))
            .pos(self.pos, self.anchor)
            .scale(
                self.scale
                    + Vector2::repeat(self.state.hover_time / 2.0).component_mul(&self.scale),
            );

        if sprite.is_hovered(ctx) {
            self.state.hover_time += ctx.delta_time;
        } else {
            self.state.hover_time -= ctx.delta_time;
        }
        self.state.hover_time = self.state.hover_time.min(0.1).max(0.0);

        ctx.draw(sprite);
    }
}
