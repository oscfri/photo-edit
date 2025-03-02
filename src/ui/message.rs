#[derive(Debug, Clone, Copy)]
pub enum MouseState {
    Up,
    Down
}

#[derive(Debug, Clone)]
pub enum MouseMessage {
    Over,
    Press,
    RightPress,
    Release,
    Scroll(f32),
}

impl Into<Message> for MouseMessage {
    fn into(self) -> Message {
        Message::WorkspaceMessage(WorkspaceMessage::ImageMouseMessage(self))
    }
}

#[derive(Debug, Clone)]
pub enum WorkspaceMessage {
    ToggleCropMode,
    ToggleMaskMode(usize),
    BrightnessChanged(f32),
    ContrastChanged(f32),
    TintChanged(f32),
    TemperatureChanged(f32),
    SaturationChanged(f32),
    AddMask,
    DeleteMask(usize),
    MaskToggleLinear(usize, bool),
    MaskBrightnessChanged(usize, f32),
    MaskAngleChanged(usize, f32),
    AngleChanged(f32),
    ExportImage,
    ImageMouseMessage(MouseMessage),
}

impl Into<Message> for WorkspaceMessage {
    fn into(self) -> Message {
        Message::WorkspaceMessage(self)
    }
}

#[derive(Debug, Clone)]
pub enum AlbumMessage {
    LoadAlbum,
    SaveAlbum,
    NextImage,
    DeleteImage,
    SetImage(usize),
}

impl Into<Message> for AlbumMessage {
    fn into(self) -> Message {
        Message::AlbumMessage(self)
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    AlbumMessage(AlbumMessage),
    WorkspaceMessage(WorkspaceMessage)
}