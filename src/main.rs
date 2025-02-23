mod album;
mod types;
mod pipeline;
mod repository;
mod update;
mod view_mode;
mod workspace;
mod ui;

use iced;
use pipeline::viewport::{self, ViewportWorkspace};
use view_mode::ViewMode;
use workspace::workspace::Workspace;
use workspace::workspace_factory::WorkspaceFactory;
use repository::repository_factory;
use ui::message::{Message, MouseMessage, MouseState};
use ui::window::Window;
use viewport::Viewport;
use iced::widget::shader::wgpu;
use futures_executor;
use pipeline::pipeline_factory::PipelineFactory;

pub fn main() -> iced::Result {
    iced::application("A cool image editor", Main::update, Main::view)
        .theme(|_| iced::Theme::Nord)
        .resizable(true)
        .run()
}

#[derive(Debug, Clone, Copy, Default)]
struct Point {
    x: i32,
    y: i32
}

struct Main {
    workspace: Workspace,

    viewport: Viewport,
    mouse_position: Point,
    mouse_state: MouseState
}

async fn test_thing(workspace: &ViewportWorkspace) {
    let wgpu_instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::all(),
        ..Default::default()
    });
    
    let adapter = wgpu_instance
        .request_adapter(&wgpu::RequestAdapterOptions::default())
        .await
        .unwrap();

    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default()
            },
            None,
        )
        .await
        .unwrap();

    // TODO: Figure out a way to bring image size...
    let bounds = pipeline::transform::Rectangle {
        center_x: 128.0,
        center_y: 128.0,
        width: 256.0,
        height: 256.0,
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

    let target = device.create_texture(
        &wgpu::TextureDescriptor {
            label: Some("target"),
            size: wgpu::Extent3d {
                width: workspace.get_image_width() as u32,
                height: workspace.get_image_height() as u32,
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
    {
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
                bytes_per_row: Some(256 * 4),
                rows_per_image: None
            }
        },
        wgpu::Extent3d {
            width: 256,
            height: 256,
            depth_or_array_layers: 1
        });

    queue.submit(Some(encoder.finish()));

    let buffer = pipeline.output_texture_buffer;
    let capturable = buffer.clone();
    buffer.slice(..).map_async(wgpu::MapMode::Read, move |result| {
        if result.is_ok() {
            let mapped_range = capturable.slice(..).get_mapped_range();
            let view: Vec<u32> = mapped_range
                .chunks_exact(4)
                .map(|b| u32::from_ne_bytes(b.try_into().unwrap()))
                .collect();

            println!("Hello: {:?}", view.len());

            drop(view);
            capturable.unmap();
        }
    });
}

impl<'a> Main {

    fn new() -> Self {

        let mut repository = repository_factory::RepositoryFactory::new().create().unwrap();
        repository.print_albums().unwrap(); // Just for demo

        let workspace: Workspace = WorkspaceFactory::new(&mut repository).create();
        let viewport: Viewport = Viewport::new(&workspace);
        let mouse_position: Point = Point::default();
        let mouse_state: MouseState = MouseState::Up;

        // let viewport_workspace = viewport::ViewportWorkspace::new(&workspace);
        // futures_executor::block_on(test_thing(&viewport_workspace));

        Self {
            workspace,
            viewport,
            mouse_position,
            mouse_state
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        let window: Window<'_> = Window::new(&self.workspace, &self.viewport, &self.mouse_position);
        window.view()
    }
}

impl Default for Main {
    fn default() -> Self {
        Self::new()
    }
}