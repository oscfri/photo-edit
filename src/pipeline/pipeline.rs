use crate::pipeline::camera_uniform;

use iced::widget::shader::wgpu;

use crate::types::RawImage;

use super::{crop_uniform, parameter_uniform};

pub struct Pipeline {
    pipeline: wgpu::RenderPipeline,
    vertices: wgpu::Buffer,
    camera_uniform: wgpu::Buffer,
    parameter_uniform: wgpu::Buffer,
    crop_uniform: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,
    diffuse_texture: wgpu::Texture,
    diffuse_bind_group: wgpu::BindGroup
}

impl Pipeline {
    pub fn new(
            pipeline: wgpu::RenderPipeline,
            vertices: wgpu::Buffer,
            camera_uniform: wgpu::Buffer,
            parameter_uniform: wgpu::Buffer,
            crop_uniform: wgpu::Buffer,
            uniform_bind_group: wgpu::BindGroup,
            diffuse_texture: wgpu::Texture,
            diffuse_bind_group: wgpu::BindGroup) -> Self {
        Self {
            pipeline,
            vertices,
            camera_uniform,
            parameter_uniform,
            crop_uniform,
            uniform_bind_group,
            diffuse_texture,
            diffuse_bind_group
        }
    }

    pub fn update(
            &self,
            queue: &wgpu::Queue,
            image: &RawImage,
            camera_uniform: &camera_uniform::CameraUniform,
            parameter_uniform: &parameter_uniform::ParameterUniform,
            crop_uniform: &crop_uniform::CropUniform) {
        let texture_size = wgpu::Extent3d {
            width: image.width as u32,
            height: image.height as u32,
            depth_or_array_layers: 1
        };
        queue.write_buffer(&self.camera_uniform, 0, bytemuck::bytes_of(camera_uniform));
        queue.write_buffer(&self.parameter_uniform, 0, bytemuck::bytes_of(parameter_uniform));
        queue.write_buffer(&self.crop_uniform, 0, bytemuck::bytes_of(crop_uniform));
        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &self.diffuse_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &image.pixels,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * image.width as u32),
                rows_per_image: Some(image.height as u32)
            },
            texture_size
        );
    }

    pub fn render(
            &self,
            encoder: &mut wgpu::CommandEncoder,
            target: &wgpu::TextureView,
            viewport: iced::Rectangle<u32>) {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("viewport"),
            color_attachments: &[Some(
                wgpu::RenderPassColorAttachment {
                    view: target,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    }
                }
            )],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None
        });
        pass.set_scissor_rect(viewport.x, viewport.y, viewport.width, viewport.height);
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &self.uniform_bind_group, &[]);
        pass.set_bind_group(1, &self.diffuse_bind_group, &[]);
        pass.set_vertex_buffer(0, self.vertices.slice(..));
        pass.draw(0..6, 0..1);
    }
}