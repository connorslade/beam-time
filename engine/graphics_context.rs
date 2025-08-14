use std::rc::Rc;

use nalgebra::Vector2;
use wgpu::Color;

use crate::{
    application::{input::InputManager, window::WindowManager},
    assets::{SpriteRef, manager::AssetManager},
    audio::AudioManager,
    color::Rgb,
    drawable::Drawable,
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

    pub input: &'a mut InputManager,
    pub window: &'a mut WindowManager,

    /// List of sprites to render this frame
    pub sprites: Vec<GpuSprite>,
    /// List of shapes to render this frame (triangulated)
    pub shapes: GpuPolygons,
    /// Background color
    pub(crate) background: Rgb<f32>,
    /// Functions to run after main render function completes
    pub(crate) defer: Vec<DeferCallback>,

    /// The time elapsed since the last frame
    pub delta_time: f32,
    /// Which frame is currently being rendered (starting from zero)
    pub frame: u64,
}

impl<'a> GraphicsContext<'a> {
    pub fn sprite_count(&self) -> usize {
        self.sprites.len()
    }

    pub fn size(&self) -> Vector2<f32> {
        self.window.size
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
