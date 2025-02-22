mod album;
mod types;
mod pipeline;
mod repository;
mod update;
mod view_mode;
mod workspace;
mod ui;

use iced;
use pipeline::viewport;
use view_mode::ViewMode;
use workspace::Workspace;
use workspace::workspace_factory::WorkspaceFactory;
use repository::repository_factory;
use ui::message::{Message, MouseMessage, MouseState};
use ui::window::Window;

pub fn main() -> iced::Result {
    iced::application("A cool image editor", Main::update, Main::view)
        .theme(|_| iced::Theme::Nord)
        .resizable(true)
        .run()
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32
}

struct Main {
    workspace: Workspace,

    mouse_position: Point,
    mouse_state: MouseState,

    viewport: viewport::Viewport
}

impl<'a> Main {

    fn new() -> Self {
        let mut repository = repository_factory::RepositoryFactory::new().create().unwrap();
        let workspace = WorkspaceFactory::new(&mut repository).create();

        repository.print_albums().unwrap(); // Just for demo

        let mouse_position: Point = Point {
            x: 0,
            y: 0
        };
        let viewport = update::make_viewport(&workspace);
        let mouse_state: MouseState = MouseState::Up;

        Self {
            workspace,
            mouse_position,
            mouse_state,
            viewport
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