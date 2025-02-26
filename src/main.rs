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

impl<'a> Main {

    fn new() -> Self {

        let mut repository = repository_factory::RepositoryFactory::new().create().unwrap();
        repository.print_albums().unwrap(); // Just for demo

        let workspace: Workspace = WorkspaceFactory::new(&mut repository).create();
        let viewport: Viewport = Viewport::new(&workspace);
        let mouse_position: Point = Point::default();
        let mouse_state: MouseState = MouseState::Up;

        let viewport_workspace = viewport::ViewportWorkspace::new(&workspace);
        futures_executor::block_on(pipeline::export_image::export_image(&viewport_workspace));

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