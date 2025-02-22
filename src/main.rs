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
use workspace::WorkSpace;
use repository::repository_factory;
use std::path::PathBuf;
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
    workspace: WorkSpace,

    mouse_position: Point,
    view_mode: ViewMode,
    mouse_state: MouseState,

    viewport: viewport::Viewport
}

impl<'a> Main {

    fn new() -> Self {
        let mut repository = repository_factory::RepositoryFactory::new().create().unwrap();

        repository.print_albums().unwrap();

        let file_names = repository.get_album_photos(0).unwrap().iter()
            .map(|album_photo| album_photo.file_name.clone())
            .map(PathBuf::from)
            .collect();

        let workspace: WorkSpace = workspace::load_workspace(&file_names);

        let mouse_position: Point = Point {
            x: 0,
            y: 0
        };
        let mode: view_mode::ViewMode = view_mode::ViewMode::Normal;
        let viewport = update::make_viewport(&workspace, &mode);
        let mouse_state: MouseState = MouseState::Up;

        Self {
            workspace,
            mouse_position,
            view_mode: mode,
            mouse_state,
            viewport
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        let window: Window<'_> = Window::compose(
            self.workspace.album_images(),
            &self.viewport,
            &self.mouse_position,
            &self.view_mode,
            self.workspace.current_parameters(),
            self.workspace.current_crop().angle_degrees);
        window.view()
    }
}

impl Default for Main {
    fn default() -> Self {
        Self::new()
    }
}