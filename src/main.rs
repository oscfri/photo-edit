mod types;
mod pipeline;
mod repository;
mod update;
mod view_mode;
mod workspace;
mod ui;

use std::path::PathBuf;
use std::sync::Arc;

use iced;
use pipeline::viewport;
use repository::repository::Repository;
use rusqlite::Connection;
use ui::welcome_window::WelcomeWindow;
use view_mode::ViewMode;
use workspace::album::Album;
use workspace::album_image_loader::AlbumImageLoader;
use workspace::workspace::Workspace;
use workspace::album_factory::AlbumFactory;
use repository::repository_factory;
use ui::message::{Message, MouseMessage, MouseState};
use ui::main_window::MainWindow;
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
    album: Album,
    workspace: Option<Workspace>,

    repository: Arc<Repository>,
    album_factory: Arc<AlbumFactory>,

    viewport: Option<Viewport>,
    mouse_position: Point,
}

impl Main {

    fn new() -> Self {

        let connection: Connection = Connection::open(PathBuf::from("album.sqlite")).unwrap();
        let repository = Arc::new(repository_factory::RepositoryFactory::new(connection).create());
        let album_image_loader = Arc::new(AlbumImageLoader::new());
        let album_factory = Arc::new(AlbumFactory::new(repository.clone(), album_image_loader.clone()));
        let album = album_factory.create();
        let workspace = album.make_workspace();
        
        let viewport = workspace.as_ref().map(Viewport::new);
        let mouse_position: Point = Point::default();

        Self {
            album,
            workspace,
            repository,
            album_factory,
            viewport,
            mouse_position,
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        if let Some(workspace) = &self.workspace {
            // TODO: Figure out how to do with viewport here
            let window: MainWindow<'_> = MainWindow::new(
                &self.album,
                &workspace,
                self.viewport.as_ref().unwrap(),
                &self.mouse_position);
            window.view()
        } else {
            let window: WelcomeWindow = WelcomeWindow::new();
            window.view()
        }
    }
}

impl Default for Main {
    fn default() -> Self {
        Self::new()
    }
}