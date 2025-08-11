use engine::{
    exports::winit::{event::MouseButton, window::CursorIcon},
    graphics_context::GraphicsContext,
    layout::tracker::LayoutTracker,
    memory::MemoryKey,
};

use crate::assets::{BUTTON_CLICK, BUTTON_HOVER};

pub struct ManualButton {
    key: MemoryKey,

    hover: bool,
    updated: bool,
}

#[derive(Default)]
struct ButtonState {
    hover_time: f32,
    last_hovered: bool,
}

impl ManualButton {
    pub fn new(key: MemoryKey) -> Self {
        Self {
            key,
            hover: false,
            updated: false,
        }
    }

    pub fn hovered(mut self, hover: bool) -> Self {
        self.hover |= hover;
        self
    }

    pub fn tracker(self, ctx: &GraphicsContext, tracker: LayoutTracker) -> Self {
        self.hovered(tracker.hovered(ctx))
    }

    pub fn tick(self, ctx: &mut GraphicsContext) -> Self {
        self._tick(ctx);
        self
    }

    // kinda jank but whatever
    fn _tick<'a>(&self, ctx: &'a mut GraphicsContext) -> &'a mut ButtonState {
        let state = ctx.memory.get_or_insert(self.key, ButtonState::default());
        if self.updated {
            return state;
        }

        state.hover_time += ctx.delta_time * if self.hover { 1.0 } else { -1.0 };
        state.hover_time = state.hover_time.clamp(0.0, 0.1);

        self.hover.then(|| ctx.window.cursor(CursorIcon::Pointer));
        if self.hover && !state.last_hovered {
            ctx.audio.builder(BUTTON_HOVER).with_gain(0.2).play_now();
        }
        state.last_hovered = self.hover;

        if self.hover && ctx.input.mouse_pressed(MouseButton::Left) {
            ctx.audio.builder(BUTTON_CLICK).play_now();
        }

        state
    }

    pub fn is_hovered(&self) -> bool {
        self.hover
    }

    pub fn hover_time(&self, ctx: &mut GraphicsContext) -> f32 {
        let state = self._tick(ctx);
        state.hover_time / 0.1
    }

    pub fn pressed(&self, ctx: &mut GraphicsContext) -> bool {
        self._tick(ctx);
        self.hover && ctx.input.mouse_pressed(MouseButton::Left)
    }
}
