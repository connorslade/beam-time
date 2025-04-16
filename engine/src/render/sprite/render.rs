use bytemuck::NoUninit;
use nalgebra::{Vector2, Vector3};
use wgpu::{BufferDescriptor, BufferUsages, Device, IndexFormat, Queue, RenderPass};

use crate::graphics_context::GraphicsContext;

use super::pipeline::SpriteRenderPipeline;

#[derive(NoUninit, Clone, Copy)]
#[repr(C)]
pub struct Instance {
    points: [Vector2<f32>; 4],
    texture: u32,
    uv: [Vector2<f32>; 2],
    layer: f32,
    color: Vector3<f32>,
    clip: [Vector2<f32>; 2],
}

impl SpriteRenderPipeline {
    pub fn prepare(&mut self, device: &Device, queue: &Queue, ctx: &GraphicsContext) {
        let window = ctx.size();
        let mut instances = Vec::new(); // todo don't realloc every frame
        for sprite in ctx.sprites.iter() {
            let layer = (i16::MAX as f32 - sprite.z_index as f32) / (i16::MAX as f32 * 2.0);
            instances.push(Instance {
                points: sprite.points.map(|x| x.component_div(&window)),
                layer,
                texture: sprite.texture.reference,
                uv: [sprite.uv[0], sprite.uv[1] - sprite.uv[0]],
                color: sprite.color.into(),
                clip: sprite.clip.map(|x| x.component_div(&window)),
            });

            let contents = bytemuck::cast_slice(&instances);
            let content_len = contents.len() as u64;

            if content_len > self.instances.size() {
                let size = content_len.next_power_of_two();
                self.instances = device.create_buffer(&BufferDescriptor {
                    label: None,
                    usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
                    mapped_at_creation: false,
                    size,
                });
            }

            queue.write_buffer(&self.instances, 0, contents);
            self.instance_count = instances.len() as u32;
        }
    }

    pub fn paint<'a>(&'a self, render_pass: &mut RenderPass<'a>) {
        render_pass.set_pipeline(&self.render_pipeline);

        render_pass.set_bind_group(0, &self.bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.instances.slice(..));
        render_pass.set_index_buffer(self.index.slice(..), IndexFormat::Uint16);
        render_pass.draw_indexed(0..6, 0, 0..self.instance_count);
    }
}
