mod album;
mod types;
mod pipeline;
mod repository;
mod update;
mod view_mode;
mod workspace;
mod ui;

use std::future::IntoFuture;

use iced;
use pipeline::viewport;
use view_mode::ViewMode;
use workspace::workspace::Workspace;
use workspace::workspace_factory::WorkspaceFactory;
use repository::repository_factory;
use ui::message::{Message, MouseMessage, MouseState};
use ui::window::Window;
use viewport::Viewport;
use wgpu;
use futures_executor;

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

async fn test_thing() {
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
                required_limits: wgpu::Limits::default(),
                memory_hints: wgpu::MemoryHints::default()
            },
            None,
        )
        .await
        .unwrap();
}

impl<'a> Main {

    fn new() -> Self {

        futures_executor::block_on(test_thing());

        let mut repository = repository_factory::RepositoryFactory::new().create().unwrap();
        repository.print_albums().unwrap(); // Just for demo

        let workspace = WorkspaceFactory::new(&mut repository).create();
        let viewport: Viewport = Viewport::from_workspace(&workspace);
        let mouse_position: Point = Point::default();
        let mouse_state: MouseState = MouseState::Up;

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