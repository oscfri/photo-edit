use crate::{pipeline::viewport, ui::message::{ImageSelectionMessage, MainParameterMessage, MaskChangeMessage, MaskMessage, Message, MiscMessage, MouseMessage, RenderMessage, ToolboxMessage, WelcomeMessage}};

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

impl Into<UpdatEvent> for MouseEvent {
    fn into(self) -> UpdatEvent {
        UpdatEvent::WorkspaceMessage(WorkspaceEvent::ImageMouseEvent(self))
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

impl Into<UpdatEvent> for WorkspaceEvent {
    fn into(self) -> UpdatEvent {
        UpdatEvent::WorkspaceMessage(self)
    }
}

#[derive(Debug, Clone)]
pub enum AlbumEvent {
    LoadAlbum,
    SaveAlbum,
    NextImage,
    DeleteImage,
    SetImage(usize),
}

impl Into<UpdatEvent> for AlbumEvent {
    fn into(self) -> UpdatEvent {
        UpdatEvent::AlbumMessage(self)
    }
}

pub enum UpdatEvent {
    WorkspaceMessage(WorkspaceEvent),
    AlbumMessage(AlbumEvent)
}

impl From<MainParameterMessage> for UpdatEvent {
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

impl From<MaskMessage> for UpdatEvent {
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

impl From<MiscMessage> for UpdatEvent {
    fn from(message: MiscMessage) -> Self {
        match message {
            MiscMessage::AngleChanged(angle) => WorkspaceEvent::AngleChanged(angle).into(),
            MiscMessage::DeleteImage => AlbumEvent::DeleteImage.into(),
            MiscMessage::ExportImage => WorkspaceEvent::ExportImage.into(),
            MiscMessage::LoadAlbum => AlbumEvent::LoadAlbum.into(),
            MiscMessage::NextImage => AlbumEvent::NextImage.into(),
            MiscMessage::SaveAlbum => AlbumEvent::SaveAlbum.into(),
            MiscMessage::ToggleCropMode => WorkspaceEvent::ToggleCropMode.into()
        }
    }
}

impl From<ToolboxMessage> for UpdatEvent {
    fn from(message: ToolboxMessage) -> Self {
        match message {
            ToolboxMessage::MainParameterMessage(message) => UpdatEvent::from(message),
            ToolboxMessage::MaskMessage(message) => UpdatEvent::from(message),
            ToolboxMessage::MiscMessage(message) => UpdatEvent::from(message),
        }
    }
}

impl From<MouseMessage> for UpdatEvent {
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

impl From<RenderMessage> for UpdatEvent {
    fn from(message: RenderMessage) -> Self {
        match message {
            RenderMessage::MouseMessage(message) => UpdatEvent::from(message)
        }
    }
}

impl From<ImageSelectionMessage> for UpdatEvent {
    fn from(message: ImageSelectionMessage) -> Self {
        match message {
            ImageSelectionMessage::SelectImage(index) => AlbumEvent::SetImage(index).into()
        }
    }
}

impl From<WelcomeMessage> for UpdatEvent {
    fn from(message: WelcomeMessage) -> Self {
        match message {
            WelcomeMessage::LoadAlbum => AlbumEvent::LoadAlbum.into()
        }
    }
}

impl From<Message> for UpdatEvent {
    fn from(message: Message) -> Self {
        match message {
            Message::ToolboxMessage(message) => UpdatEvent::from(message),
            Message::RenderMessage(message) => UpdatEvent::from(message),
            Message::ImageSelectionMessage(message) => UpdatEvent::from(message),
            Message::WelcomeMessage(message) => UpdatEvent::from(message)
        }
    }
}