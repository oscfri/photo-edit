#![windows_subsystem = "windows"]
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

use directories::ProjectDirs;
use iced;
use pipeline::viewport;
use repository::repository::Repository;
use rusqlite::Connection;
use ui::welcome_window::WelcomeWindow;
use view_mode::ViewMode;
use workspace::album::Album;
use workspace::image_manager::ImageManager;
use workspace::workspace::Workspace;
use repository::repository_factory;
use ui::message::{KeyboardMessage, Message, MouseState};
use ui::main_window::MainWindow;
use viewport::Viewport;

pub fn main() -> iced::Result {
    iced::application("A cool image editor", Main::update, Main::view)
        .theme(|_| iced::Theme::Nord)
        .font(iced_fonts::NERD_FONT_BYTES)
        .resizable(true)
        .subscription(Main::subscription)
        .exit_on_close_request(false) // Allows for auto save on close
        .run_with(init)
}

struct Main {
    album: Album,
    workspace: Option<Workspace>,

    repository: Arc<Repository>,
    image_manager: ImageManager,

    viewport: Option<Viewport>,
}

fn init() -> (Main, iced::Task<Message>) {
    (Main::new(), iced::Task::done(Message::OnStartMessage))
}

fn handle_keyboard_event(event: iced::keyboard::Event) -> Option<KeyboardMessage> {
    match event {
        iced::keyboard::Event::KeyPressed { key, modified_key: _, physical_key: _, location: _, modifiers: _, text: _ } => {
            handle_key_press(key)
        },
        _ => None
    }
}

fn handle_key_press(key: iced::keyboard::Key) -> Option<KeyboardMessage> {
    match key.as_ref() {
        iced::keyboard::Key::Character(character) => {
            handle_key_press_character(character)
        },
        _ => None
    }
}

fn handle_key_press_character(character: &str) -> Option<KeyboardMessage> {
    match character {
        "d" => Some(KeyboardMessage::NextImage),
        "a" => Some(KeyboardMessage::PreviousImage),
        "q" => Some(KeyboardMessage::CropRotateLeft),
        "e" => Some(KeyboardMessage::CropRotateRight),
        "f" => Some(KeyboardMessage::ToggleFavorite),
        "g" => Some(KeyboardMessage::ToggleDisplayGrid),
        "c" => Some(KeyboardMessage::ToggleCropMode),
        _ => None
    }
}

impl Main {
    fn new() -> Self {
        let db_path: PathBuf = Self::create_db_path();

        let connection: Connection = Connection::open(db_path).unwrap();
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

    fn create_db_path() -> PathBuf {
        let config_dir = ProjectDirs::from("com", "Photo Editor", "Photo Editor")
            .unwrap()
            .config_dir()
            .to_path_buf();

        std::fs::create_dir_all(&config_dir).unwrap();
        
        config_dir.join("album.db")
    }

    pub fn view(&self) -> iced::Element<Message> {
        if let Some(workspace) = &self.workspace {
            let window: MainWindow<'_> = MainWindow::new(
                &self.image_manager,
                &self.album,
                &workspace,
                &self.viewport);
            window.view()
        } else {
            let window: WelcomeWindow = WelcomeWindow::new();
            window.view()
        }
    }

    pub fn subscription(&self) -> iced::Subscription<Message> {
        iced::Subscription::batch(vec![
            iced::event::listen_with(|event, _status, _window| match event {
                iced::Event::Keyboard(keyboard_event) => handle_keyboard_event(keyboard_event).map(Message::KeyboardMessage),
                _ => None
            }),
            iced::window::close_requests().map(Message::OnWindowCloseMessage),
            iced::time::every(Duration::from_secs(10)).map(|_| Message::OnTimeTickMessage),
        ])
    }
}