use std::{cell::RefCell, mem, u32};

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
    scale: f32,

    width: f32,
    default: &'static str,
    max_chars: u32,
    key: MemoryKey,

    text: RefCell<Option<Text>>,
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
            scale: 2.0,

            width: 0.0,
            default: "",
            max_chars: u32::MAX,
            key,

            text: RefCell::new(None),
        }
    }

    pub fn default(self, default: &'static str) -> Self {
        Self { default, ..self }
    }

    pub fn position(self, position: Vector2<f32>, position_anchor: Anchor) -> Self {
        Self {
            position,
            position_anchor,
            ..self
        }
    }

    pub fn z_index(self, z_index: i16) -> Self {
        Self { z_index, ..self }
    }

    pub fn width(self, width: f32) -> Self {
        self.invalidate_text();
        Self { width, ..self }
    }

    pub fn max_chars(self, max_chars: u32) -> Self {
        Self { max_chars, ..self }
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

impl TextInput {
    fn invalidate_text(&self) {
        self.text.replace(None);
    }

    fn generate_text(&self, ctx: &mut GraphicsContext) {
        if self.text.borrow().is_some() {
            return;
        }

        let padding = 4.0 * ctx.scale_factor;
        let state = ctx
            .memory
            .get_or_insert_with(self.key, || TextInputState::new(self.default.into()));

        let text = Text::new(UNDEAD_FONT, &state.content)
            .position(
                self.position + Vector2::repeat(padding),
                self.position_anchor,
            )
            .scale(Vector2::repeat(self.scale))
            .z_index(self.z_index)
            .max_width(self.width);

        self.text.replace(Some(text));
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
        let padding = 4.0 * ctx.scale_factor;

        self.generate_text(ctx);
        let text = self.text.take().unwrap();
        let size = text.size(ctx);
        text.draw(ctx);

        let state = ctx
            .memory
            .get_or_insert_with(self.key, || TextInputState::new(self.default.into()));
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
                if (state.content.len() as u32) < self.max_chars {
                    state.content.push_str(text.as_str());
                }
            }
        }

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
        self.invalidate_text();
        self.position += distance;
    }

    fn bounds(&self, ctx: &mut GraphicsContext) -> Bounds2D {
        self.generate_text(ctx);
        let size = self.text.borrow().as_ref().unwrap().size(ctx);

        let padding = 4.0 * ctx.scale_factor;
        let pos = self.position - Vector2::repeat(padding);
        let offset = padding * 3.0 + ctx.scale_factor * 2.0;
        Bounds2D::new(pos, pos + Vector2::new(self.width, size.y + offset))
    }

    fn draw(self: Box<Self>, ctx: &mut GraphicsContext) {
        (*self).draw(ctx);
    }
}
