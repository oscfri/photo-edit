mod album;
mod types;
mod pipeline;
mod repository;
mod update;
mod view_mode;
mod workspace;
mod ui;

use std::path::PathBuf;

use album::Album;
use iced;
use pipeline::viewport;
use repository::repository::Repository;
use rusqlite::Connection;
use view_mode::ViewMode;
use workspace::album_image_loader::AlbumImageLoader;
use workspace::workspace::Workspace;
use workspace::album_factory::AlbumFactory;
use repository::repository_factory;
use ui::message::{Message, MouseMessage, MouseState};
use ui::window::Window;
use viewport::Viewport;

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
    repository: Repository,

    viewport: Viewport,
    mouse_position: Point,
    mouse_state: MouseState
}

impl<'a> Main {

    fn new() -> Self {

        let connection: Connection = Connection::open(PathBuf::from("album.sqlite")).unwrap();
        let repository = repository_factory::RepositoryFactory::new(connection).create().unwrap();
        repository.print_albums().unwrap(); // Just for demo

        let album_image_loader: AlbumImageLoader = AlbumImageLoader::new();
        let album: Album = AlbumFactory::new(&repository, &album_image_loader).create();
        let workspace: Workspace = Workspace::new(album);
        let viewport: Viewport = Viewport::new(&workspace);
        let mouse_position: Point = Point::default();
        let mouse_state: MouseState = MouseState::Up;

        Self {
            workspace,
            repository,
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