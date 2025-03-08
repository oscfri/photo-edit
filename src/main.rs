mod types;
mod pipeline;
mod repository;
mod update;
mod update_event;
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
use ui::message::{Message, MouseState};
use ui::main_window::MainWindow;
use viewport::Viewport;

pub fn main() -> iced::Result {
    iced::application("A cool image editor", Main::update, Main::view)
        .theme(|_| iced::Theme::Nord)
        .resizable(true)
        .run_with(init)
}

struct Main {
    album: Album,
    workspace: Option<Workspace>,

    repository: Arc<Repository>,
    album_factory: Arc<AlbumFactory>,

    viewport: Option<Viewport>,
}

fn init() -> (Main, iced::Task<Message>) {
    let connection: Connection = Connection::open(PathBuf::from("album.sqlite")).unwrap();
    let repository = Arc::new(repository_factory::RepositoryFactory::new(connection).create());
    let album_image_loader = Arc::new(AlbumImageLoader::new());
    let album_factory = Arc::new(AlbumFactory::new(repository.clone(), album_image_loader.clone()));
    let album = album_factory.create();
    let workspace = album.make_workspace();
    
    let viewport = workspace.as_ref().and_then(Viewport::try_new);

    let main = Main {
        album,
        workspace,
        repository,
        album_factory,
        viewport,
    };
    (main, iced::Task::done(Message::OnStartMessage))
}

impl Main {
    pub fn view(&self) -> iced::Element<Message> {
        if let Some(workspace) = &self.workspace {
            let window: MainWindow<'_> = MainWindow::new(
                &self.album,
                &workspace,
                &self.viewport);
            window.view()
        } else {
            let window: WelcomeWindow = WelcomeWindow::new();
            window.view()
        }
    }
}