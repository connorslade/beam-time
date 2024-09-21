use engine::{
    assets::AssetRef,
    color::Rgb,
    drawable::sprites::Sprite,
    exports::{nalgebra::Vector2, winit::event::MouseButton},
    graphics_context::{Anchor, Drawable, GraphicsContext},
};

use crate::consts::ACCENT_COLOR;

pub struct Button<'a> {
    asset: AssetRef,
    state: &'a mut ButtonState,
    on_click: Box<dyn FnMut(&mut GraphicsContext)>,

    color: Rgb<f32>,
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
            on_click: Box::new(|_| {}),

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

    pub fn color(mut self, color: impl Into<Rgb<f32>>) -> Self {
        self.color = color.into();
        self
    }

    pub fn scale(mut self, scale: Vector2<f32>) -> Self {
        self.scale = scale;
        self
    }

    pub fn on_click(mut self, on_click: impl FnMut(&mut GraphicsContext) + 'static) -> Self {
        self.on_click = Box::new(on_click);
        self
    }
}

impl<'a> Drawable for Button<'a> {
    fn draw(mut self, ctx: &mut GraphicsContext) {
        let color = self.color.lerp(ACCENT_COLOR, self.state.hover_time / 0.1);
        let scale =
            self.scale + Vector2::repeat(self.state.hover_time / 2.0).component_mul(&self.scale);

        let sprite = Sprite::new(self.asset)
            .color(color)
            .position(self.pos, self.anchor)
            .scale(scale, Anchor::Center);

        let hover = sprite.is_hovered(ctx);
        self.state.hover_time += ctx.delta_time * if hover { 1.0 } else { -1.0 };
        self.state.hover_time = self.state.hover_time.clamp(0.0, 0.1);

        if hover && ctx.input.mouse_down(MouseButton::Left) {
            (self.on_click)(ctx);
        }

        ctx.draw(sprite);
    }
}
