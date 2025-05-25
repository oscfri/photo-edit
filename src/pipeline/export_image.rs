use std::path::{Path, PathBuf};

use iced::widget::shader::wgpu;

use crate::workspace::workspace::Workspace;

use super::pipeline::Pipeline;
use super::viewport::ViewportWorkspace;
use super::transform::Rectangle;
use super::pipeline_factory::PipelineFactory;

pub const EXPORT_SIZE: u32 = 8192;

// TODO: This should be done in a separate thread...
pub async fn export_image(workspace: &Workspace, export_directory: PathBuf) {
    if let Some(viewport_workspace) = ViewportWorkspace::try_new(&workspace) {
        let file_name = workspace.get_file_name();
        export_image_from_viewport(viewport_workspace, export_directory, file_name).await
    }
}

async fn export_image_from_viewport(viewport_workspace: ViewportWorkspace, export_directory: PathBuf, file_name: String) {
    let (device, queue) = request_device().await.unwrap();
    
    // TODO: Figure out a way to bring image size...
    let bounds = Rectangle {
        center_x: (EXPORT_SIZE / 2) as f32,
        center_y: (EXPORT_SIZE / 2) as f32,
        width: EXPORT_SIZE as f32,
        height: EXPORT_SIZE as f32,
        angle_degrees: 0.0
    };

    let pipline_factory = PipelineFactory::new(
        viewport_workspace.get_image_width(),
        viewport_workspace.get_image_height(),
        &device,
        wgpu::TextureFormat::Rgba8UnormSrgb);

    let pipeline = pipline_factory.create();
    pipeline.update(&queue, &viewport_workspace, &bounds, &bounds, 1.0);

    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

    render(&mut encoder, &device, &pipeline);

    encoder.copy_texture_to_buffer(
        wgpu::ImageCopyTexture {
            texture: &pipeline.output_texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        wgpu::ImageCopyBufferBase {
            buffer: &pipeline.output_texture_buffer,
            layout: wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(EXPORT_SIZE * 4),
                rows_per_image: None
            }
        },
        wgpu::Extent3d {
            width: EXPORT_SIZE,
            height: EXPORT_SIZE,
            depth_or_array_layers: 1
        });

    queue.submit(Some(encoder.finish()));

    let width = viewport_workspace.parameters.crop.width as u32;
    let height = viewport_workspace.parameters.crop.height as u32;
    let buffer = pipeline.output_texture_buffer;
    let capturable = buffer.clone();
    buffer.slice(..)
        .map_async(wgpu::MapMode::Read, move |result| {
            if result.is_ok() {
                let mapped_range = capturable.slice(..).get_mapped_range();
                let view: Vec<u32> = mapped_range
                    .chunks_exact(4)
                    .map(|b| u32::from_ne_bytes(b.try_into().unwrap()))
                    .collect();

                let path = Path::join(&export_directory, "image.jpg")
                    .with_file_name(file_name)
                    .with_extension("jpg");
                write_image(&view, &path, width, height);

                drop(mapped_range);
                capturable.unmap();
            }
        });
}

async fn request_device() -> Result<(wgpu::Device, wgpu::Queue), wgpu::RequestDeviceError> {
    // TODO: Feels excessive to request device for each export. Is it worth it to keep the device?
    let wgpu_instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::all(),
        ..Default::default()
    });
    
    let adapter = wgpu_instance
        .request_adapter(&wgpu::RequestAdapterOptions::default())
        .await
        .unwrap();

    adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default()
            },
            None,
        )
        .await
}

fn render(encoder: &mut wgpu::CommandEncoder, device: &wgpu::Device, pipeline: &Pipeline) {
    let target = device.create_texture(
        &wgpu::TextureDescriptor {
            label: Some("target"),
            size: wgpu::Extent3d {
                width: EXPORT_SIZE, // TODO: Not sure what good values are here. This needs to be large to prevent artifacts
                height: EXPORT_SIZE, // Is there a way to guarantee no artifacts?
                depth_or_array_layers: 1
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        }
    );
    let target_view = target.create_view(&wgpu::TextureViewDescriptor::default());
    let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("viewport"),
        color_attachments: &[Some(
            wgpu::RenderPassColorAttachment {
                view: &target_view,
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

    pipeline.render_pass(&mut pass);
}

fn write_image(data: &Vec<u32>, path: &PathBuf, width: u32, height: u32) {
    let mut rgb_data: Vec<u8> = Vec::with_capacity((width * height * 3) as usize);

    let mut x = 0;
    let mut y = 0;
    for d in data {
        if x < width && y < height {
            let red: u8 = (d & 0xff) as u8;
            let green: u8 = ((d >> 8) & 0xff) as u8;
            let blue: u8 = ((d >> 16) & 0xff) as u8;
            rgb_data.push(red);
            rgb_data.push(green);
            rgb_data.push(blue);
        }

        x += 1;
        if x >= EXPORT_SIZE {
            x = 0;
            y += 1;
        }
    }

    let image: image::RgbImage = image::RgbImage::from_raw(width, height, rgb_data).unwrap();
    
    image.save_with_format(path, image::ImageFormat::Jpeg).unwrap();
}