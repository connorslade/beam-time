use std::{cell::RefCell, mem};

use engine::{
    color::Rgb,
    drawable::{Anchor, Drawable},
    drawable::{
        shape::{rectangle::Rectangle, rectangle_outline::RectangleOutline},
        text::Text,
    },
    exports::{
        nalgebra::Vector2,
        winit::{
            event::{ElementState, MouseButton},
            keyboard::{KeyCode, PhysicalKey},
        },
    },
    graphics_context::GraphicsContext,
    layout::{LayoutElement, bounds::Bounds2D},
    memory::{Memory, MemoryKey},
};

use crate::assets::UNDEAD_FONT;

pub struct TextInput {
    position: Vector2<f32>,
    position_anchor: Anchor,
    z_index: i16,
    scale: f32,

    default_content: &'static str,
    default_active: bool,
    width: f32,
    max_chars: u32,
    key: MemoryKey,

    text: RefCell<Option<Text>>,
}

#[derive(Default)]
pub struct TextInputState {
    content: String,
    selected: bool,
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

            default_content: "",
            default_active: false,
            max_chars: u32::MAX,
            width: 0.0,
            key,

            text: RefCell::new(None),
        }
    }

    pub fn default_active(self, default_active: bool) -> Self {
        Self {
            default_active,
            ..self
        }
    }

    pub fn placeholder(self, default_content: &'static str) -> Self {
        Self {
            default_content,
            ..self
        }
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
}

impl TextInput {
    pub fn content(&self, ctx: &mut GraphicsContext) -> String {
        Self::content_for(ctx, self.key)
    }

    pub fn content_for(ctx: &mut GraphicsContext, key: MemoryKey) -> String {
        let state = ctx.memory.get::<TextInputState>(key);
        if let Some(TextInputState {
            content,
            unedited: false,
            ..
        }) = state
        {
            content.to_owned()
        } else {
            String::new()
        }
    }

    pub fn with_content(&self, ctx: &mut GraphicsContext, content: String) {
        let state = self.state(ctx.memory);
        state.unedited = content.is_empty();
        state.content = content;
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
        let state = self.state(ctx.memory);

        let text = Text::new(UNDEAD_FONT, &state.content)
            .position(Vector2::repeat(padding), self.position_anchor)
            .scale(Vector2::repeat(self.scale))
            .z_index(self.z_index)
            .max_width(self.width);

        self.text.replace(Some(text));
    }

    fn state<'a>(&self, memory: &'a mut Memory) -> &'a mut TextInputState {
        memory.get_or_insert_with(self.key, || TextInputState {
            content: self.default_content.into(),
            selected: self.default_active,
            unedited: true,
            ..Default::default()
        })
    }
}

impl Drawable for TextInput {
    fn draw(self, ctx: &mut GraphicsContext) {
        let padding = 4.0 * ctx.scale_factor;

        let hovered = self.bounds(ctx).contains(ctx.input.mouse());
        let state = self.state(ctx.memory);

        if ctx.input.mouse_pressed(MouseButton::Left) {
            state.t = if hovered { 0.0 } else { state.t };
            state.selected = hovered;
        }

        state.t += ctx.delta_time;
        let (t, selected) = (state.t, state.selected);
        let color = if state.unedited {
            Rgb::repeat(0.80)
        } else {
            Rgb::repeat(1.0)
        };

        let text = self.text.take().unwrap();
        let text = text.color(color).position(
            self.position + Vector2::repeat(padding),
            self.position_anchor,
        );

        let layout = text.scaled_layout(ctx);
        let (size, cursor) = (layout.size, layout.ending_position);
        drop(layout);
        text.draw(ctx);

        RectangleOutline::new(Vector2::new(self.width, size.y + padding * 3.0), 2.0)
            .position(
                self.position - Vector2::repeat(padding),
                self.position_anchor,
            )
            .color(Rgb::repeat(0.75))
            .draw(ctx);

        if !selected {
            return;
        };

        let state = self.state(ctx.memory);
        let actions = ctx.input.consume_key_actions();
        for key in actions
            .into_iter()
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
            } else if let (Some(text), true) =
                (&key.text, (state.content.len() as u32) < self.max_chars)
            {
                state.content.push_str(text.as_str());
            }
        }

        if (t * 4.0).cos() > 0.0 {
            let font_desc = &ctx.assets.get_font(UNDEAD_FONT).desc;
            let font_height = font_desc.height * self.scale * ctx.scale_factor;
            let pos = self.position + Vector2::x() * padding + cursor;
            Rectangle::new(Vector2::new(
                2.0 * ctx.scale_factor,
                font_height + padding * 2.0,
            ))
            .position(pos, self.position_anchor)
            .draw(ctx);
        }
    }
}

impl LayoutElement for TextInput {
    fn translate(&mut self, distance: Vector2<f32>) {
        self.position += distance;
    }

    fn bounds(&self, ctx: &mut GraphicsContext) -> Bounds2D {
        self.generate_text(ctx);
        let size = self.text.borrow().as_ref().unwrap().size(ctx);

        let padding = 4.0 * ctx.scale_factor;
        let pos = -Vector2::repeat(padding);
        let offset = padding * 3.0 + ctx.scale_factor * 2.0;

        Bounds2D::new(pos, pos + Vector2::new(self.width, size.y + offset))
            .translated(self.position)
    }

    fn draw(self: Box<Self>, ctx: &mut GraphicsContext) {
        (*self).draw(ctx);
    }
}
