use std::path::PathBuf;

use iced::widget::shader::wgpu;

use crate::view_mode::ViewMode;

use super::pipeline::Pipeline;
use super::viewport::ViewportWorkspace;
use super::transform::Rectangle;
use super::pipeline_factory::PipelineFactory;

// TODO: This should be done in a separate thread...
pub async fn export_image(workspace: &ViewportWorkspace) {
    let (device, queue) = request_device().await.unwrap();
    
    // TODO: Figure out a way to bring image size...
    let bounds = Rectangle {
        center_x: 1024.0,
        center_y: 1024.0,
        width: 2048.0,
        height: 2048.0,
        angle_degrees: 0.0
    };

    let pipline_factory = PipelineFactory::new(
        workspace.get_image_width(),
        workspace.get_image_height(),
        &device,
        wgpu::TextureFormat::Rgba8UnormSrgb);

    let pipeline = pipline_factory.create();
    pipeline.update(&queue, workspace, &ViewMode::Normal, &bounds, &bounds);

    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

    render(&mut encoder, &device, &pipeline, workspace);

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
                bytes_per_row: Some(2048 * 4),
                rows_per_image: None
            }
        },
        wgpu::Extent3d {
            width: 2048,
            height: 2048,
            depth_or_array_layers: 1
        });

    queue.submit(Some(encoder.finish()));

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

                write_image(&view, &PathBuf::from("test.jpg"));

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

fn render(encoder: &mut wgpu::CommandEncoder, device: &wgpu::Device, pipeline: &Pipeline, workspace: &ViewportWorkspace) {
    let target = device.create_texture(
        &wgpu::TextureDescriptor {
            label: Some("target"),
            size: wgpu::Extent3d {
                width: 2048 as u32,
                height: 2048 as u32,
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

fn write_image(data: &Vec<u32>, path: &PathBuf) {
    let mut rgb_data: Vec<u8> = Vec::with_capacity(2048 * 2048 * 3);

    for d in data {
        let red: u8 = (d & 0xff) as u8;
        let green: u8 = ((d >> 8) & 0xff) as u8;
        let blue: u8 = ((d >> 16) & 0xff) as u8;
        rgb_data.push(red);
        rgb_data.push(green);
        rgb_data.push(blue);
    }

    println!("Data: {}", rgb_data.len());

    let image: image::RgbImage = image::RgbImage::from_raw(2048, 2048, rgb_data).unwrap();

    println!("Size: {}, {}", image.width(), image.height());
    image.save_with_format(path, image::ImageFormat::Jpeg).unwrap();
}