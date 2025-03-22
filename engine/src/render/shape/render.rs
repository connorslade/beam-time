use bytemuck::{Pod, Zeroable};
use nalgebra::Vector3;
use wgpu::{BufferDescriptor, BufferUsages, Device, IndexFormat, Queue, RenderPass};

use crate::{graphics_context::GraphicsContext, render::layer_to_z_coord};

use super::pipeline::ShapeRenderPipeline;

#[derive(Pod, Zeroable, Clone, Copy)]
#[repr(C)]
pub struct GpuShapeVertex {
    position: Vector3<f32>,
    color: Vector3<f32>,
}

impl ShapeRenderPipeline {
    pub fn prepare<App>(&mut self, device: &Device, queue: &Queue, ctx: &GraphicsContext<App>) {
        let size = ctx.size();
        let verts = ctx
            .shapes
            .vertices
            .iter()
            .map(|x| {
                let z = layer_to_z_coord(x.z_index);
                GpuShapeVertex {
                    position: x.position.component_div(&size).push(z),
                    color: x.color.into(),
                }
            })
            .collect::<Vec<_>>();

        let verts = bytemuck::cast_slice(&verts);
        let index = bytemuck::cast_slice(&ctx.shapes.indices);
        self.count = ctx.shapes.indices.len() as u32;

        if verts.len() as u64 > self.vertex.size() {
            self.vertex = device.create_buffer(&BufferDescriptor {
                label: None,
                size: verts.len() as u64,
                usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
        } else {
            queue.write_buffer(&self.vertex, 0, verts);
        }

        if index.len() as u64 > self.index.size() {
            self.index = device.create_buffer(&BufferDescriptor {
                label: None,
                size: index.len() as u64,
                usage: BufferUsages::INDEX | BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
        } else {
            queue.write_buffer(&self.index, 0, index);
        }
    }

    pub fn paint<'a>(&'a self, render_pass: &mut RenderPass<'a>) {
        render_pass.set_pipeline(&self.render_pipeline);

        render_pass.set_bind_group(0, &self.bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex.slice(..));
        render_pass.set_index_buffer(self.index.slice(..), IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.count, 0, 0..1);
    }
}
