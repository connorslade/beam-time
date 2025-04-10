use std::mem;

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
    layout::{bounds::Bounds2D, LayoutElement},
    memory::MemoryKey,
};

use crate::assets::UNDEAD_FONT;

pub struct TextInput {
    position: Vector2<f32>,
    position_anchor: Anchor,
    z_index: i16,

    width: f32,
    scale: f32,

    key: MemoryKey,
}

#[derive(Default)]
pub struct TextInputState {
    content: String,
    unedited: bool,
    t: f32,
}

impl TextInput {
    pub fn new(key: MemoryKey) -> Self {
        Self {
            position: Vector2::zeros(),
            position_anchor: Anchor::BottomLeft,
            z_index: 0,

            width: 0.0,
            scale: 2.0,

            key,
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

    pub fn content(&self, ctx: &mut GraphicsContext) -> String {
        ctx.memory
            .get::<TextInputState>(self.key)
            .map(|x| x.content.to_owned())
            .unwrap_or_default()
    }

    pub fn content_for(ctx: &mut GraphicsContext, key: MemoryKey) -> String {
        ctx.memory
            .get::<TextInputState>(key)
            .map(|x| x.content.to_owned())
            .unwrap_or_default()
    }
}

impl TextInputState {
    pub fn new(content: String) -> Self {
        Self {
            content,
            unedited: true,
            t: 0.0,
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl Drawable for TextInput {
    fn draw(self, ctx: &mut GraphicsContext) {
        let state = ctx
            .memory
            .get_or_insert(self.key, TextInputState::default());
        state.t += ctx.delta_time;
        let t = state.t;

        for key in ctx
            .input
            .key_actions
            .iter()
            .filter(|x| x.state == ElementState::Pressed)
        {
            let backspace = key.physical_key == PhysicalKey::Code(KeyCode::Backspace);
            if backspace || key.text.is_some() {
                state.t = 0.0;
                if mem::take(&mut state.unedited) {
                    state.content.clear();
                }
            }

            if backspace {
                state.content.pop();
            } else if let Some(ref text) = key.text {
                state.content.push_str(text.as_str());
            }
        }

        let padding = 4.0 * ctx.scale_factor;

        let text = Text::new(UNDEAD_FONT, &state.content)
            .position(
                self.position + Vector2::repeat(padding),
                self.position_anchor,
            )
            .z_index(self.z_index)
            .scale(Vector2::repeat(self.scale));
        let size = text.size(ctx);
        text.draw(ctx);

        RectangleOutline::new(Vector2::new(self.width, size.y + padding * 3.0), 2.0)
            .position(
                self.position - Vector2::repeat(padding),
                self.position_anchor,
            )
            .color(Rgb::repeat(0.75))
            .draw(ctx);

        if (t * 4.0).cos() > 0.0 {
            Rectangle::new(Vector2::new(2.0 * ctx.scale_factor, size.y))
                .position(
                    self.position + Vector2::repeat(padding) + Vector2::x() * (size.x + padding),
                    self.position_anchor,
                )
                .draw(ctx);
        }
    }
}

impl LayoutElement for TextInput {
    fn translate(&mut self, distance: Vector2<f32>) {
        self.position += distance;
    }

    fn bounds(&self, ctx: &mut GraphicsContext) -> Bounds2D {
        let font = ctx.assets.get_font(UNDEAD_FONT);
        let font_height = font.desc.height * ctx.scale_factor * self.scale;

        let padding = 4.0 * ctx.scale_factor;
        let height = font_height + padding * 3.0;

        // todo: account for position anchor
        let pos = self.position - Vector2::repeat(padding);
        Bounds2D::new(pos, pos + Vector2::new(self.width, height))
    }

    fn draw(self: Box<Self>, ctx: &mut GraphicsContext) {
        (*self).draw(ctx);
    }
}
