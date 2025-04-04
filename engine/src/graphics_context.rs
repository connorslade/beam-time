use std::rc::Rc;

use nalgebra::Vector2;
use wgpu::Color;
use winit::window::Cursor;

use crate::{
    assets::{manager::AssetManager, SpriteRef},
    audio::AudioManager,
    color::Rgb,
    input::InputManager,
    memory::Memory,
    render::{
        shape::{GpuPolygons, ShapeVertex},
        sprite::GpuSprite,
    },
};

type DeferCallback = Box<dyn FnOnce(&mut GraphicsContext)>;

pub struct GraphicsContext<'a> {
    /// Reference to asset manager
    pub assets: Rc<AssetManager>,
    pub audio: &'a AudioManager,
    pub memory: &'a mut Memory,

    /// Background color
    pub(crate) background: Rgb<f32>,
    /// List of sprites to render this frame
    pub(crate) sprites: Vec<GpuSprite>,
    /// List of shapes to render this frame (triangulated)
    pub(crate) shapes: GpuPolygons,
    /// The cursor to use for the next frame
    pub(crate) cursor: Cursor,
    /// Functions to run after main render function completes
    pub(crate) defer: Vec<DeferCallback>,

    pub input: &'a mut InputManager,
    /// Current window scale_factor
    pub scale_factor: f32,
    /// The time elapsed since the last frame
    pub delta_time: f32,
    /// Which frame is currently being rendered (starting from zero)
    pub frame: u64,
}

pub trait Drawable {
    fn draw(self, ctx: &mut GraphicsContext);
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

impl<'a> GraphicsContext<'a> {
    pub fn new(
        assets: Rc<AssetManager>,
        scale_factor: f32,
        input: &'a mut InputManager,
        audio: &'a AudioManager,
        memory: &'a mut Memory,
        delta_time: f32,
        frame: u64,
    ) -> Self {
        GraphicsContext {
            assets,
            audio,
            memory,
            background: Rgb::new(0.0, 0.0, 0.0),
            sprites: Vec::new(),
            shapes: Default::default(),
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

    pub fn defer(&mut self, callback: impl FnOnce(&mut GraphicsContext) + 'static) {
        self.defer.push(Box::new(callback));
    }

    pub fn draw(&mut self, drawable: impl Drawable) {
        drawable.draw(self);
    }

    pub fn draw_callback(
        &mut self,
        drawable: impl FnOnce(&mut GraphicsContext),
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

impl GraphicsContext<'_> {
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

impl<T: Drawable, const N: usize> Drawable for [T; N] {
    fn draw(self, ctx: &mut GraphicsContext) {
        self.into_iter().for_each(|x| ctx.draw(x));
    }
}
