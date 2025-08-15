use std::{mem, sync::Arc};

use nalgebra::Vector2;
use winit::{
    event::WindowEvent,
    window::{Cursor, Fullscreen, Window},
};

use crate::misc::Mutable;

pub struct WindowManager {
    window: Arc<Window>,
    user_scale: f32,

    /// The size of the window in logical pixels.
    size: Vector2<f32>,
    scale_factor: f32,
    close_next: bool,
    close: bool,

    size_changed: Option<Vector2<f32>>,
    dpi_changed: Option<f32>,
    focus_change: Option<bool>,

    pub(crate) vsync: Mutable<bool>,
    pub(crate) fullscreen: Mutable<bool>,
    pub(crate) cursor: Mutable<Cursor>,
    pub(crate) scale: Mutable<f32>,
}

impl WindowManager {
    pub(crate) fn new(window: Arc<Window>) -> Self {
        let size = window.inner_size();
        let scale_factor = window.scale_factor() as f32;

        Self {
            window,
            user_scale: 1.0,

            size: Vector2::new(size.width, size.height).map(|x| x as f32) / scale_factor,
            scale_factor,
            close_next: false,
            close: false,

            size_changed: None,
            dpi_changed: None,
            focus_change: None,

            vsync: Mutable::default(),
            fullscreen: Mutable::default(),
            cursor: Mutable::default(),
            scale: Mutable::default(),
        }
    }

    pub(crate) fn on_window_event(&mut self, window_event: &WindowEvent) {
        match window_event {
            WindowEvent::CloseRequested => self.close = true,
            WindowEvent::Focused(focused) => self.focus_change = Some(*focused),
            WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                self.dpi_changed = Some(self.scale_factor);
                self.scale_factor = *scale_factor as f32 * self.user_scale;
            }
            WindowEvent::Resized(size) => {
                self.size_changed = Some(self.size);

                let physical_size = Vector2::new(size.width, size.height);
                self.size = physical_size.map(|x| x as f32) / self.scale_factor;
            }
            _ => {}
        }
    }

    pub(crate) fn on_frame_end(&mut self) {
        self.vsync.desired();
        if let Some(cursor) = self.cursor.desired() {
            self.window.set_cursor(cursor.clone());
        }

        if let Some(fullscreen) = self.fullscreen.desired() {
            self.window
                .set_fullscreen(fullscreen.then_some(Fullscreen::Borderless(None)));
        }

        self.cursor.set(Default::default());
        self.close |= mem::take(&mut self.close_next);
        self.size_changed = None;
        self.dpi_changed = None;
        self.focus_change = None;

        if let Some(&scale) = self.scale.desired() {
            self.dpi_changed = Some(self.scale_factor);
            self.scale_factor = self.scale_factor / self.user_scale * scale;

            self.size_changed = Some(self.size);
            self.size = self.size * self.user_scale / scale;

            self.user_scale = scale;
        }
    }
}

impl WindowManager {
    #[inline(always)]
    pub fn close(&mut self) {
        self.close_next = true;
    }

    #[inline(always)]
    pub fn vsync(&mut self, vsync: bool) {
        self.vsync.set(vsync);
    }

    #[inline(always)]
    pub fn fullscreen(&mut self, fullscreen: bool) {
        self.fullscreen.set(fullscreen);
    }

    #[inline(always)]
    pub fn cursor(&mut self, cursor: impl Into<Cursor>) {
        self.cursor.set(cursor.into());
    }

    pub fn user_scale(&mut self, scale: f32) {
        self.scale.set(scale);
    }
}

impl WindowManager {
    #[inline(always)]
    pub fn size(&self) -> Vector2<f32> {
        self.size
    }

    #[inline(always)]
    pub fn size_changed(&self) -> Option<Vector2<f32>> {
        self.size_changed
    }

    #[inline(always)]
    pub fn delta_size(&self) -> Vector2<f32> {
        if let Some(resized) = self.size_changed {
            self.size - resized
        } else {
            Vector2::zeros()
        }
    }

    #[inline(always)]
    pub fn close_requested(&self) -> bool {
        self.close
    }

    #[inline(always)]
    pub fn resized(&self) -> Option<Vector2<f32>> {
        self.size_changed
    }

    #[inline(always)]
    pub fn dpi_changed(&self) -> Option<f32> {
        self.dpi_changed
    }

    #[inline(always)]
    pub fn just_focused(&self) -> bool {
        self.focus_change == Some(true)
    }

    #[inline(always)]
    pub fn scale_factor(&self) -> f32 {
        self.scale_factor
    }
}
