use engine::{
    exports::{nalgebra::Vector2, winit::event::MouseButton},
    graphics_context::GraphicsContext,
};

pub mod beam;
pub mod board;
pub mod tile;

pub struct SharedState {
    pub pan: Vector2<f32>,
    pub scale: f32,
    scale_goal: f32,
}

impl SharedState {
    pub fn update<App>(&mut self, ctx: &GraphicsContext<App>) {
        self.scale_goal = (self.scale_goal + ctx.input.scroll_delta).max(1.0);
        self.scale += (self.scale_goal - self.scale) * 10.0 * ctx.delta_time;
        if ctx.input.mouse_down(MouseButton::Middle) {
            self.pan += ctx.input.mouse_delta;
        }
    }

    pub fn tile_pos<App>(
        &self,
        ctx: &GraphicsContext<App>,
        size: Vector2<usize>,
        pos: Vector2<usize>,
    ) -> Vector2<f32> {
        let tile_size = 16.0 * self.scale * ctx.scale_factor;
        let size = size.map(|x| x as f32) * tile_size;

        ctx.center() - pos.map(|x| x as f32) * tile_size + size / 2.0
            - Vector2::repeat(tile_size / 2.0)
            + self.pan
    }
}

impl Default for SharedState {
    fn default() -> Self {
        Self {
            pan: Vector2::zeros(),
            scale_goal: 4.0,
            scale: 4.0,
        }
    }
}
