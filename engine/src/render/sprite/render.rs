use std::collections::HashMap;

use bytemuck::NoUninit;
use wgpu::{BufferDescriptor, BufferUsages, Device, IndexFormat, Queue, RenderPass};

use crate::{assets::TextureRef, graphics_context::GraphicsContext};

use super::{pipeline::SpriteRenderPipeline, GpuSprite};

#[derive(NoUninit, Clone, Copy)]
#[repr(C)]
pub struct Instance {
    points: [[f32; 2]; 4],
    uv: [[f32; 2]; 2],
    layer: f32,
    color: [f32; 3],
    clip: [[f32; 2]; 2],
}

impl SpriteRenderPipeline {
    pub fn prepare<App>(&mut self, device: &Device, queue: &Queue, ctx: &GraphicsContext<App>) {
        let mut atlases = HashMap::<TextureRef, Vec<&GpuSprite>>::new();

        for sprite in ctx.sprites.iter() {
            atlases.entry(sprite.texture).or_default().push(sprite);
        }

        // clear atlas lists
        for val in self.atlases.values_mut() {
            val.instance_count = 0;
        }

        let window = ctx.size();
        for (atlas, sprites) in atlases.iter() {
            let mut instances = Vec::new(); // todo don't realloc every frame
            for sprite in sprites.iter() {
                let layer = (i16::MAX as f32 - sprite.z_index as f32) / (i16::MAX as f32 * 2.0);
                instances.push(Instance {
                    points: [
                        sprite.points[0].component_div(&window).into(),
                        sprite.points[1].component_div(&window).into(),
                        sprite.points[2].component_div(&window).into(),
                        sprite.points[3].component_div(&window).into(),
                    ],
                    layer,
                    uv: [sprite.uv[0].into(), (sprite.uv[1] - sprite.uv[0]).into()],
                    color: sprite.color.into(),
                    clip: [
                        sprite.clip[0].component_div(&window).into(),
                        sprite.clip[1].component_div(&window).into(),
                    ],
                });
            }

            let contents = bytemuck::cast_slice(&instances);
            let content_len = contents.len() as u64;

            let operation = self.atlases.get_mut(atlas).unwrap();

            if content_len > operation.instances.size() {
                let size = content_len.next_power_of_two();
                operation.instances = device.create_buffer(&BufferDescriptor {
                    label: None,
                    usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
                    mapped_at_creation: false,
                    size,
                });
            }

            queue.write_buffer(&operation.instances, 0, contents);
            operation.instance_count = instances.len() as u32;
        }
    }

    pub fn paint<'a>(&'a self, render_pass: &mut RenderPass<'a>) {
        render_pass.set_pipeline(&self.render_pipeline);

        for operation in self.atlases.values() {
            render_pass.set_bind_group(0, &operation.bind_group, &[]);
            render_pass.set_vertex_buffer(0, operation.instances.slice(..));
            render_pass.set_index_buffer(self.index.slice(..), IndexFormat::Uint16);
            render_pass.draw_indexed(0..6, 0, 0..operation.instance_count);
        }
    }
}
