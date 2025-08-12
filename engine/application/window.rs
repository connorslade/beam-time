use std::{mem, sync::Arc};

use nalgebra::Vector2;
use winit::{
    event::WindowEvent,
    window::{Cursor, Fullscreen, Window},
};

use crate::misc::Mutable;

pub struct WindowManager {
    window: Arc<Window>,

    pub(crate) size: Vector2<u32>,
    scale_factor: f32,
    close_next: bool,
    close: bool,

    size_changed: Option<Vector2<u32>>,
    dpi_changed: Option<f32>,
    focus_change: Option<bool>,

    pub(crate) vsync: Mutable<bool>,
    pub(crate) fullscreen: Mutable<bool>,
    pub(crate) cursor: Mutable<Cursor>,
}

impl WindowManager {
    pub(crate) fn new(window: Arc<Window>) -> Self {
        let size = window.inner_size();

        Self {
            window,

            size: Vector2::new(size.width, size.height),
            scale_factor: 1.0,
            close_next: false,
            close: false,

            size_changed: None,
            dpi_changed: None,
            focus_change: None,

            vsync: Mutable::default(),
            fullscreen: Mutable::default(),
            cursor: Mutable::default(),
        }
    }

    pub(crate) fn on_window_event(&mut self, window_event: &WindowEvent) {
        match window_event {
            WindowEvent::CloseRequested => self.close = true,
            WindowEvent::Focused(focused) => self.focus_change = Some(*focused),
            WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                self.dpi_changed = Some(self.scale_factor);
                self.scale_factor = *scale_factor as f32;
            }
            WindowEvent::Resized(size) => {
                self.size_changed = Some(self.size);
                self.size = Vector2::new(size.width, size.height);
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
}

impl WindowManager {
    #[inline(always)]
    pub fn size_changed(&self) -> Option<Vector2<f32>> {
        self.size_changed.map(|x| x.map(|x| x as f32))
    }

    #[inline(always)]
    pub fn delta_size(&self) -> Vector2<f32> {
        if let Some(resized) = self.size_changed {
            self.size.map(|x| x as f32) - resized.map(|x| x as f32)
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
        self.size_changed.map(|x| x.map(|x| x as f32))
    }

    #[inline(always)]
    pub fn dpi_changed(&self) -> Option<f32> {
        self.dpi_changed
    }

    #[inline(always)]
    pub fn just_focused(&self) -> bool {
        self.focus_change == Some(true)
    }
}
