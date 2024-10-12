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
        let old_scale = self.scale;
        self.scale_goal = (self.scale_goal + ctx.input.scroll_delta).max(1.0);
        self.scale += (self.scale_goal - self.scale) * 10.0 * ctx.delta_time;

        // Scale around the curser position, not the world origin. Don't ask how
        // long this took me to get right...
        let scale_center = ctx.input.mouse;
        self.pan += (scale_center - self.pan) * (old_scale - self.scale) / old_scale;

        if ctx.input.mouse_down(MouseButton::Middle) {
            self.pan += ctx.input.mouse_delta;
        }
    }

    pub fn origin_tile<App>(&self, ctx: &GraphicsContext<App>) -> Vector2<i32> {
        (self.pan / (16.0 * self.scale * ctx.scale_factor)).map(|x| x.floor() as i32)
    }

    pub fn render_pos<App>(
        &self,
        ctx: &GraphicsContext<App>,
        (x, y): (usize, usize),
    ) -> Vector2<f32> {
        let tile_size = 16.0 * self.scale * ctx.scale_factor;
        let half_tile = Vector2::repeat(tile_size / 2.0);

        let pan_offset = (self.pan / tile_size).map(|x| x.fract()) * tile_size;
        let tile_offset = Vector2::new(
            -tile_size * (self.pan.x >= 0.0) as u8 as f32,
            -tile_size * (self.pan.y >= 0.0) as u8 as f32,
        );

        Vector2::new(x as f32, y as f32) * tile_size + pan_offset + tile_offset + half_tile
    }

    pub fn tile_counts(&self, size: Vector2<f32>) -> Vector2<usize> {
        (size / (16.0 * self.scale)).map(|x| 1 + x.ceil() as usize)
    }

    pub fn tile_pos<App>(
        &self,
        ctx: &GraphicsContext<App>,
        (x, y): (usize, usize),
    ) -> Vector2<i32> {
        Vector2::new(x as i32, y as i32) - self.origin_tile(ctx)
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
