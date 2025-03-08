use crate::{pipeline::viewport, types::RawImage, ui::message::{BottomPaneMessage, ImageSelectionMessage, MainParameterMessage, MaskChangeMessage, MaskMessage, Message, MiscMessage, MouseMessage, RenderMessage, TaskMessage, ToolboxMessage, TopPaneMessage, WelcomeMessage}};

#[derive(Debug, Clone, Copy)]
pub struct MousePosition {
    pub image_x: i32,
    pub image_y: i32,
    pub relative_x: i32,
    pub relative_y: i32
}

#[derive(Debug, Clone, Copy)]
pub enum MouseEvent {
    Press(MousePosition),
    Release,
    RightPress(MousePosition),
    Over(MousePosition),
    Scroll(f32)
}

impl Into<UpdateEvent> for MouseEvent {
    fn into(self) -> UpdateEvent {
        UpdateEvent::WorkspaceEvent(WorkspaceEvent::ImageMouseEvent(self))
    }
}

#[derive(Debug, Clone)]
pub enum WorkspaceEvent {
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
    ImageMouseEvent(MouseEvent),
}

impl Into<UpdateEvent> for WorkspaceEvent {
    fn into(self) -> UpdateEvent {
        UpdateEvent::WorkspaceEvent(self)
    }
}

#[derive(Debug, Clone)]
pub enum AlbumEvent {
    LoadAlbum,
    SaveAlbum,
    NextImage,
    PreviousImage,
    DeleteImage,
    SetImage(usize),
    LoadImage(i32, RawImage, RawImage)
}

impl Into<UpdateEvent> for AlbumEvent {
    fn into(self) -> UpdateEvent {
        UpdateEvent::AlbumEvent(self)
    }
}

pub enum UpdateEvent {
    OnStart,
    WorkspaceEvent(WorkspaceEvent),
    AlbumEvent(AlbumEvent)
}

impl From<BottomPaneMessage> for UpdateEvent {
    fn from(message: BottomPaneMessage) -> Self {
        match message {
            BottomPaneMessage::NextImage => AlbumEvent::NextImage.into(),
            BottomPaneMessage::PreviousImage => AlbumEvent::PreviousImage.into(),
            BottomPaneMessage::DeleteImage => AlbumEvent::DeleteImage.into(),
        }
    }
}

impl From<MainParameterMessage> for UpdateEvent {
    fn from(message: MainParameterMessage) -> Self {
        match message {
            MainParameterMessage::BrightnessChanged(brightness) => WorkspaceEvent::BrightnessChanged(brightness).into(),
            MainParameterMessage::ContrastChanged(contrast) => WorkspaceEvent::ContrastChanged(contrast).into(),
            MainParameterMessage::SaturationChanged(saturation) => WorkspaceEvent::SaturationChanged(saturation).into(),
            MainParameterMessage::TemperatureChanged(temperature) => WorkspaceEvent::TemperatureChanged(temperature).into(),
            MainParameterMessage::TintChanged(tint) => WorkspaceEvent::TintChanged(tint).into()
        }
    }
}

impl From<MaskMessage> for UpdateEvent {
    fn from(message: MaskMessage) -> Self {
        match message {
            MaskMessage::AddMask => WorkspaceEvent::AddMask.into(),
            MaskMessage::MaskChanged(mask_index, message) => {
                match message {
                    MaskChangeMessage::MaskAngleChanged(angle) => WorkspaceEvent::MaskAngleChanged(mask_index, angle).into(),
                    MaskChangeMessage::BrightnessChanged(brightness) => WorkspaceEvent::MaskBrightnessChanged(mask_index, brightness).into(),
                    MaskChangeMessage::MaskToggleLinear(toggle) => WorkspaceEvent::MaskToggleLinear(mask_index, toggle).into(),
                    MaskChangeMessage::DeleteMask => WorkspaceEvent::DeleteMask(mask_index).into(),
                    MaskChangeMessage::ToggleMaskMode => WorkspaceEvent::ToggleMaskMode(mask_index).into()
                }
            }
        }
    }
}

impl From<MiscMessage> for UpdateEvent {
    fn from(message: MiscMessage) -> Self {
        match message {
            MiscMessage::AngleChanged(angle) => WorkspaceEvent::AngleChanged(angle).into(),
            MiscMessage::SaveAlbum => AlbumEvent::SaveAlbum.into(),
            MiscMessage::ToggleCropMode => WorkspaceEvent::ToggleCropMode.into()
        }
    }
}

impl From<ToolboxMessage> for UpdateEvent {
    fn from(message: ToolboxMessage) -> Self {
        match message {
            ToolboxMessage::MainParameterMessage(message) => UpdateEvent::from(message),
            ToolboxMessage::MaskMessage(message) => UpdateEvent::from(message),
            ToolboxMessage::MiscMessage(message) => UpdateEvent::from(message),
        }
    }
}

impl From<TopPaneMessage> for UpdateEvent {
    fn from(message: TopPaneMessage) -> Self {
        match message {
            TopPaneMessage::LoadAlbum => AlbumEvent::LoadAlbum.into(),
            TopPaneMessage::Export => WorkspaceEvent::ExportImage.into()
        }
    }
}

impl From<MouseMessage> for UpdateEvent {
    fn from(message: MouseMessage) -> Self {
        let image_mouse_x: i32 = viewport::get_image_mouse_x();
        let image_mouse_y: i32 = viewport::get_image_mouse_y();
        let relative_mouse_x: i32 = viewport::get_relative_mouse_x();
        let relative_mouse_y: i32 = viewport::get_relative_mouse_y();
        let mouse_position: MousePosition = MousePosition {
            image_x: image_mouse_x, 
            image_y: image_mouse_y,
            relative_x: relative_mouse_x,
            relative_y: relative_mouse_y
        };
        match message {
            MouseMessage::Over => MouseEvent::Over(mouse_position).into(),
            MouseMessage::Press => MouseEvent::Press(mouse_position).into(),
            MouseMessage::Release => MouseEvent::Release.into(),
            MouseMessage::RightPress => MouseEvent::RightPress(mouse_position).into(),
            MouseMessage::Scroll(scroll) => MouseEvent::Scroll(scroll).into()
        }
    }
}

impl From<RenderMessage> for UpdateEvent {
    fn from(message: RenderMessage) -> Self {
        match message {
            RenderMessage::MouseMessage(message) => UpdateEvent::from(message)
        }
    }
}

impl From<ImageSelectionMessage> for UpdateEvent {
    fn from(message: ImageSelectionMessage) -> Self {
        match message {
            ImageSelectionMessage::SelectImage(index) => AlbumEvent::SetImage(index).into()
        }
    }
}

impl From<WelcomeMessage> for UpdateEvent {
    fn from(message: WelcomeMessage) -> Self {
        match message {
            WelcomeMessage::LoadAlbum => AlbumEvent::LoadAlbum.into()
        }
    }
}

impl From<TaskMessage> for UpdateEvent {
    fn from(message: TaskMessage) -> Self {
        match message {
            TaskMessage::NewImage(image_load_result) => {
                let photo_id = image_load_result.photo_id;
                let image = image_load_result.image;
                let thumbnail = image_load_result.thumbnail;
                AlbumEvent::LoadImage(photo_id, image, thumbnail).into()
            }
        }
    }
}

impl From<Message> for UpdateEvent {
    fn from(message: Message) -> Self {
        match message {
            Message::OnStartMessage => UpdateEvent::OnStart,
            Message::BottomPaneMessage(message) => UpdateEvent::from(message),
            Message::ImageSelectionMessage(message) => UpdateEvent::from(message),
            Message::RenderMessage(message) => UpdateEvent::from(message),
            Message::ToolboxMessage(message) => UpdateEvent::from(message),
            Message::TopPaneMessage(message) => UpdateEvent::from(message),
            Message::WelcomeMessage(message) => UpdateEvent::from(message),
            Message::TaskMessage(message) => UpdateEvent::from(message)
        }
    }
}