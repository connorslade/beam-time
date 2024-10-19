use engine::{
    exports::{
        nalgebra::Vector2,
        winit::{event::MouseButton, keyboard::KeyCode, window::CursorIcon},
    },
    graphics_context::GraphicsContext,
};

use crate::app::App;

pub mod beam;
pub mod board;
mod selection;
pub mod tile;

pub struct SharedState {
    pub pan: Vector2<f32>,
    pub scale: f32,
    scale_goal: f32,
    pan_goal: Vector2<f32>,
}

const PAN_KEYS: [(KeyCode, Vector2<f32>); 4] = [
    (KeyCode::KeyW, Vector2::new(0.0, -1.0)),
    (KeyCode::KeyA, Vector2::new(1.0, 0.0)),
    (KeyCode::KeyS, Vector2::new(0.0, 1.0)),
    (KeyCode::KeyD, Vector2::new(-1.0, 0.0)),
];

impl SharedState {
    pub fn update(&mut self, ctx: &mut GraphicsContext<App>, state: &App) {
        let mut delta_pan = Vector2::zeros();

        if ctx.input.mouse_down(MouseButton::Middle) {
            ctx.set_cursor(CursorIcon::Grabbing);
            delta_pan += ctx.input.mouse_delta;
        }

        let direction = PAN_KEYS
            .iter()
            .filter(|(key, _)| ctx.input.key_down(*key))
            .map(|(_, dir)| *dir)
            .sum::<Vector2<_>>();
        if direction.magnitude_squared() != 0.0 {
            self.pan_goal +=
                direction.normalize() * self.scale * state.config.movement_speed * ctx.delta_time;
        }

        // TODO: Dont allow scale goal to be non integer values when close to 1.0
        let old_scale = self.scale;
        self.scale_goal = (self.scale_goal
            + ctx.input.scroll_delta * state.config.zoom_sensitivity)
            .clamp(1.0, 10.0);

        let lerp_speed = 10.0 * ctx.delta_time;
        self.scale += (self.scale_goal - self.scale) * lerp_speed;
        self.pan += (self.pan_goal - self.pan) * lerp_speed;

        // Scale around the curser position, not the world origin. Don't ask how
        // long this took me to get right...
        delta_pan += (ctx.input.mouse - self.pan) * (old_scale - self.scale) / old_scale;

        self.delta_pan(delta_pan);
    }

    pub fn on_resize(&mut self, old_size: Vector2<f32>, new_size: Vector2<f32>) {
        let delta_size = new_size - old_size;
        self.delta_pan(delta_size / 2.0);
    }

    fn delta_pan(&mut self, delta: Vector2<f32>) {
        self.pan += delta;
        self.pan_goal += delta;
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

    pub fn screen_to_world_space<App>(
        &self,
        ctx: &GraphicsContext<App>,
        pos: Vector2<f32>,
    ) -> Vector2<f32> {
        (pos - self.pan) / (16.0 * self.scale * ctx.scale_factor)
    }
}

impl Default for SharedState {
    fn default() -> Self {
        Self {
            pan: Vector2::zeros(),
            pan_goal: Vector2::zeros(),

            scale: 4.0,
            scale_goal: 4.0,
        }
    }
}
