use std::rc::Rc;

use nalgebra::Vector2;
use wgpu::Color;
use winit::window::Cursor;

use crate::{
    assets::{manager::AssetManager, SpriteRef},
    audio::AudioManager,
    color::Rgb,
    input::InputManager,
    render::{
        shape::{GpuPolygons, ShapeVertex},
        sprite::GpuSprite,
    },
    screens::Screen,
};

pub struct GraphicsContext<'a, App> {
    /// Reference to asset manager
    pub assets: Rc<AssetManager>,
    pub audio: &'a AudioManager,

    /// Background color
    pub(crate) background: Rgb<f32>,
    /// List of sprites to render this frame
    pub(crate) sprites: Vec<GpuSprite>,
    /// List of shapes to render this frame (triangluated)
    pub(crate) shapes: GpuPolygons,
    /// Screens to open for next frame
    pub(crate) next_screen: Vec<Box<dyn Screen<App>>>,
    /// Screens to close for next_frame
    pub(crate) close_screen: usize,
    /// The cursor to use for the next frame
    pub(crate) cursor: Cursor,
    /// Functions to run after main render function completes
    pub(crate) defer: Vec<Box<dyn FnOnce(&mut GraphicsContext<App>)>>,

    pub input: &'a mut InputManager,
    /// Current window scale_factor
    pub scale_factor: f32,
    /// The time elapsed since the last frame
    pub delta_time: f32,
    /// Which frame is currently being rendered (starting from zero)
    pub frame: u64,
}

pub trait Drawable<App> {
    fn draw(self, ctx: &mut GraphicsContext<App>);
}

#[derive(Debug, Copy, Clone)]
pub enum Anchor {
    TopLeft,
    TopCenter,
    TopRight,

    CenterLeft,
    Center,
    CenterRight,

    BottomLeft,
    BottomCenter,
    BottomRight,

    Custom(Vector2<f32>),
}

impl<'a, App> GraphicsContext<'a, App> {
    pub fn new(
        assets: Rc<AssetManager>,
        scale_factor: f32,
        input: &'a mut InputManager,
        audio: &'a AudioManager,
        delta_time: f32,
        frame: u64,
    ) -> Self {
        GraphicsContext {
            assets,
            audio,
            background: Rgb::new(0.0, 0.0, 0.0),
            sprites: Vec::new(),
            shapes: GpuPolygons::new(),
            next_screen: Vec::new(),
            close_screen: 0,
            cursor: Cursor::default(),
            defer: Vec::new(),
            input,
            scale_factor,
            delta_time,
            frame,
        }
    }

    pub fn sprite_count(&self) -> usize {
        self.sprites.len()
    }

    pub fn size(&self) -> Vector2<f32> {
        self.input.window_size.map(|x| x as f32)
    }

    pub fn center(&self) -> Vector2<f32> {
        self.size() / 2.0
    }

    pub fn size_of(&self, asset: SpriteRef) -> Vector2<f32> {
        self.assets.get_sprite(asset).size.map(|x| x as f32)
    }

    pub fn background(&mut self, color: Rgb<f32>) {
        self.background = color;
    }

    pub fn push_screen(&mut self, screen: impl Screen<App> + 'static) {
        self.next_screen.push(Box::new(screen));
    }

    pub fn pop_screen(&mut self) {
        self.close_screen += 1;
    }

    pub fn defer(&mut self, callback: impl FnOnce(&mut GraphicsContext<App>) + 'static) {
        self.defer.push(Box::new(callback));
    }

    pub fn draw(&mut self, drawable: impl Drawable<App>) {
        drawable.draw(self);
    }

    pub fn draw_callback(
        &mut self,
        drawable: impl FnOnce(&mut GraphicsContext<App>),
    ) -> (&mut [GpuSprite], &mut [ShapeVertex]) {
        let sprites = self.sprites.len();
        let shapes = self.shapes.vertices.len();

        drawable(self);

        (
            &mut self.sprites[sprites..],
            &mut self.shapes.vertices[shapes..],
        )
    }

    pub fn set_cursor(&mut self, cursor: impl Into<Cursor>) {
        self.cursor = cursor.into();
    }

    pub fn darken(&mut self, color: Rgb<f32>, below: i16) {
        self.background *= color;
        self.sprites
            .iter_mut()
            .filter(|sprite| sprite.z_index < below)
            .for_each(|sprite| sprite.color *= color);

        self.shapes
            .vertices
            .iter_mut()
            .filter(|vert| vert.z_index < below)
            .for_each(|vert| vert.color *= color);
    }
}

impl<App> GraphicsContext<'_, App> {
    pub(crate) fn background_color(&self) -> Color {
        Color {
            r: self.background.r as f64,
            g: self.background.g as f64,
            b: self.background.b as f64,
            a: 1.0,
        }
    }
}

impl Anchor {
    pub fn offset(&self, size: Vector2<f32>) -> Vector2<f32> {
        match self {
            Anchor::Custom(offset) => *offset,

            Anchor::CenterLeft => -Vector2::new(0.0, size.y / 2.0),
            Anchor::Center => -size / 2.0,
            Anchor::CenterRight => -Vector2::new(size.x, size.y / 2.0),

            Anchor::BottomLeft => Vector2::zeros(),
            Anchor::BottomCenter => -Vector2::new(size.x / 2.0, 0.0),
            Anchor::BottomRight => -Vector2::new(size.x, 0.0),

            Anchor::TopLeft => -Vector2::new(0.0, size.y),
            Anchor::TopCenter => -Vector2::new(size.x / 2.0, size.y),
            Anchor::TopRight => -Vector2::new(size.x, size.y),
        }
    }
}

impl<App, T: Drawable<App>, const N: usize> Drawable<App> for [T; N] {
    fn draw(self, ctx: &mut GraphicsContext<App>) {
        self.into_iter().for_each(|x| ctx.draw(x));
    }
}
