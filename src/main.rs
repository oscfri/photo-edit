mod album;
mod types;
mod pipeline;
mod workspace;
mod update;
mod view;
mod view_mode;

use iced;
use pipeline::viewport;
use view_mode::ViewMode;
use workspace::WorkSpace;
use std::path::PathBuf;

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

#[derive(Debug, Clone)]
enum MouseState {
    Up,
    Down
}

#[derive(Debug, Clone)]
enum MouseMessage {
    Over,
    Press,
    RightPress,
    Release
}

#[derive(Debug, Clone)]
enum Message {
    LoadAlbum,
    NextImage,
    SetImage(usize),
    ToggleCropMode,
    ToggleMaskMode(usize),
    BrightnessChanged(f32),
    ContrastChanged(f32),
    TintChanged(f32),
    TemperatureChanged(f32),
    SaturationChanged(f32),
    MaskBrightnessChanged(usize, f32),
    AngleChanged(f32),
    ImageMouseMessage(MouseMessage),
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
        let workspace: WorkSpace = workspace::load_workspace(&vec![
            PathBuf::from("example.png"),
            PathBuf::from("example2.jpg")
        ]);

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
}

impl Default for Main {
    fn default() -> Self {
        Self::new()
    }
}