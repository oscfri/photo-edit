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
use std::time::Duration;

use iced::{self, Subscription};
use pipeline::viewport;
use repository::repository::Repository;
use rusqlite::Connection;
use ui::welcome_window::WelcomeWindow;
use view_mode::ViewMode;
use workspace::album::Album;
use workspace::image_manager::ImageManager;
use workspace::workspace::Workspace;
use repository::repository_factory;
use ui::message::{Message, MouseState};
use ui::main_window::MainWindow;
use viewport::Viewport;

pub fn main() -> iced::Result {
    iced::application("A cool image editor", Main::update, Main::view)
        .theme(|_| iced::Theme::Nord)
        .font(iced_fonts::BOOTSTRAP_FONT_BYTES)
        .resizable(true)
        .subscription(Main::subscription)
        .exit_on_close_request(false) // Allows for auto save on close
        .run_with(|| (Main::new(), iced::Task::done(Message::OnStartMessage)))
}

struct Main {
    album: Album,
    workspace: Option<Workspace>,

    repository: Arc<Repository>,
    image_manager: ImageManager,

    viewport: Option<Viewport>,
}

impl Main {
    fn new() -> Self {
        let connection: Connection = Connection::open(PathBuf::from("album.sqlite")).unwrap();
        let repository = Arc::new(repository_factory::RepositoryFactory::new(connection).create());
        let image_manager = ImageManager::create_from(repository.clone());
        let album = Album::new(image_manager.get_all_album_images());
        let workspace = album.get_photo_id()
            .and_then(|photo_id| image_manager.get_workspace_image(photo_id))
            .map(Workspace::new);
        
        let viewport = workspace.as_ref().and_then(Viewport::try_new);
    
        Self {
            album,
            workspace,
            repository,
            image_manager,
            viewport,
        }
    }

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

    pub fn subscription(&self) -> Subscription<Message> {
        iced::Subscription::batch(vec![
            iced::window::close_requests().map(Message::OnWindowCloseMessage),
            iced::time::every(Duration::from_secs(10)).map(|_| Message::OnTimeTickMessage),
        ])
    }
}