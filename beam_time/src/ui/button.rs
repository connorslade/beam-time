use engine::{
    assets::SpriteRef,
    color::Rgb,
    drawable::sprite::Sprite,
    exports::{nalgebra::Vector2, winit::event::MouseButton},
    graphics_context::{Anchor, Drawable, GraphicsContext},
};

use crate::{
    assets::{BUTTON_BACK, BUTTON_HOVER, BUTTON_SUCCESS},
    consts::ACCENT_COLOR,
};

type ClickHandler<App> = Box<dyn FnOnce(&mut GraphicsContext<App>)>;

pub struct Button<'a, App> {
    asset: SpriteRef,
    state: &'a mut ButtonState,
    on_click: ClickHandler<App>,
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

impl<'a, App> Button<'a, App> {
    pub fn new(asset: SpriteRef, state: &'a mut ButtonState) -> Self {
        Self {
            asset,
            state,
            on_click: Box::new(|_| {}),
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

    pub fn on_click(mut self, on_click: impl FnOnce(&mut GraphicsContext<App>) + 'static) -> Self {
        self.on_click = Box::new(on_click);
        self
    }

    pub fn set_back(mut self) -> Self {
        self.is_back = true;
        self
    }
}

impl ButtonState {
    pub fn reset(&mut self) {
        self.hover_time = 0.0;
    }
}

impl<'a, App> Drawable<App> for Button<'a, App> {
    fn draw(self, ctx: &mut GraphicsContext<App>) {
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

        if hover && !self.state.last_hovered {
            ctx.audio.builder(BUTTON_HOVER).with_gain(0.2).play_now();
        }

        if hover && ctx.input.mouse_pressed(MouseButton::Left) {
            ctx.audio
                .builder(if self.is_back {
                    BUTTON_BACK
                } else {
                    BUTTON_SUCCESS
                })
                .play_now();
            (self.on_click)(ctx);
        }

        self.state.last_hovered = hover;
        ctx.draw(sprite);
    }
}
