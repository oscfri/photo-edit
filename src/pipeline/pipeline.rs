use crate::pipeline::camera_uniform;

use iced::widget::shader::wgpu::{self, RenderPass};

use super::{crop_uniform, parameter_uniform, radial_parameter, transform::Rectangle, viewport::ViewportWorkspace};

pub struct Pipeline {
    pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    camera_buffer: wgpu::Buffer,
    parameter_buffer: wgpu::Buffer,
    crop_buffer: wgpu::Buffer,
    radial_parameters_buffer: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,
    diffuse_texture: wgpu::Texture,
    diffuse_bind_group: wgpu::BindGroup,
    pub output_texture: wgpu::Texture,
    pub output_texture_buffer: std::sync::Arc<wgpu::Buffer>,
}

impl Pipeline {
    pub fn new(
            pipeline: wgpu::RenderPipeline,
            vertex_buffer: wgpu::Buffer,
            camera_buffer: wgpu::Buffer,
            parameter_buffer: wgpu::Buffer,
            crop_buffer: wgpu::Buffer,
            radial_parameters_buffer: wgpu::Buffer,
            uniform_bind_group: wgpu::BindGroup,
            diffuse_texture: wgpu::Texture,
            diffuse_bind_group: wgpu::BindGroup,
            output_texture: wgpu::Texture,
            output_texture_buffer: wgpu::Buffer) -> Self {
        let output_texture_buffer = std::sync::Arc::new(output_texture_buffer);
        Self {
            pipeline,
            vertex_buffer,
            camera_buffer,
            parameter_buffer,
            crop_buffer,
            radial_parameters_buffer,
            uniform_bind_group,
            diffuse_texture,
            diffuse_bind_group,
            output_texture,
            output_texture_buffer,
        }
    }

    pub fn update(
            &self,
            queue: &wgpu::Queue,
            workspace: &ViewportWorkspace,
            bounds: &Rectangle,
            viewport: &Rectangle,
            scale_factor: f32) {
        let camera_uniform = camera_uniform::CameraUniform::new(
                &bounds,
                &viewport,
                &workspace);
        let parameter_uniform = parameter_uniform::ParameterUniform::new(&workspace.parameters);
        let crop_uniform = crop_uniform::CropUniform::new(&workspace, &viewport, scale_factor);
        let radial_parameters = radial_parameter::RadialParameters::new(&workspace.parameters, workspace.view_mode);

        queue.write_buffer(&self.camera_buffer, 0, bytemuck::bytes_of(&camera_uniform));
        queue.write_buffer(&self.parameter_buffer, 0, bytemuck::bytes_of(&parameter_uniform));
        queue.write_buffer(&self.crop_buffer, 0, bytemuck::bytes_of(&crop_uniform));
        queue.write_buffer(&self.radial_parameters_buffer, 0, bytemuck::bytes_of(&radial_parameters));
        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &self.diffuse_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &workspace.image.pixels,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * workspace.image.width as u32),
                rows_per_image: Some(workspace.image.height as u32)
            },
            wgpu::Extent3d {
                width: workspace.image.width as u32,
                height: workspace.image.height as u32,
                depth_or_array_layers: 1
            }
        );
    }
}

impl<'a> Pipeline {
    pub fn render_pass(&'a self, pass: &mut RenderPass<'a>) {
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &self.uniform_bind_group, &[]);
        pass.set_bind_group(1, &self.diffuse_bind_group, &[]);
        pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        pass.draw(0..6, 0..1);
    }
}