use engine::{
    color::Rgb,
    drawable::{
        shape::{rectangle::Rectangle, rectangle_outline::RectangleOutline},
        text::Text,
    },
    exports::{
        nalgebra::Vector2,
        winit::{
            event::ElementState,
            keyboard::{KeyCode, PhysicalKey},
        },
    },
    graphics_context::{Anchor, Drawable, GraphicsContext},
};

use crate::assets::UNDEAD_FONT;

pub struct TextInput<'a> {
    position: Vector2<f32>,
    position_anchor: Anchor,
    z_index: i16,

    width: f32,
    scale: f32,

    state: &'a mut TextInputState,
}

#[derive(Default)]
pub struct TextInputState {
    content: String,
    t: f32,
}

impl<'a> TextInput<'a> {
    pub fn new(state: &'a mut TextInputState) -> Self {
        Self {
            position: Vector2::zeros(),
            position_anchor: Anchor::BottomLeft,
            z_index: 0,

            width: 0.0,
            scale: 2.0,

            state,
        }
    }

    pub fn position(mut self, position: Vector2<f32>, anchor: Anchor) -> Self {
        self.position = position;
        self.position_anchor = anchor;
        self
    }

    pub fn z_index(mut self, z_index: i16) -> Self {
        self.z_index = z_index;
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }
}

impl<'a, App> Drawable<App> for TextInput<'a> {
    fn draw(self, ctx: &mut GraphicsContext<App>) {
        for key in ctx
            .input
            .key_actions
            .iter()
            .filter(|x| x.state == ElementState::Pressed)
        {
            if key.physical_key == PhysicalKey::Code(KeyCode::Backspace) {
                self.state.content.pop();
                self.state.t = 0.0;
            } else if let Some(ref text) = key.text {
                self.state.content.push_str(text.as_str());
                self.state.t = 0.0;
            }
        }

        let padding = 4.0 * ctx.scale_factor;

        let text = Text::new(UNDEAD_FONT, &self.state.content)
            .position(
                self.position + Vector2::repeat(padding),
                self.position_anchor,
            )
            .z_index(self.z_index)
            .scale(Vector2::repeat(self.scale));
        let size = text.size(&ctx);

        ctx.draw(text);
        ctx.draw(
            RectangleOutline::new(Vector2::new(self.width, size.y + padding * 3.0), 4.0)
                .position(
                    self.position - Vector2::repeat(padding),
                    self.position_anchor,
                )
                .color(Rgb::repeat(0.75)),
        );

        self.state.t += ctx.delta_time;
        if (self.state.t * 4.0).cos() > 0.0 {
            ctx.draw(
                Rectangle::new(Vector2::new(2.0 * ctx.scale_factor, size.y))
                    .position(
                        self.position
                            + Vector2::repeat(padding)
                            + Vector2::x() * (size.x + padding),
                        self.position_anchor,
                    )
                    .color(Rgb::repeat(0.75)),
            );
        }
    }
}
