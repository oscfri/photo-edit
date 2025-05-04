use crate::{pipeline::viewport, types::RawImage, ui::message::{BottomPaneMessage, ImageSelectionMessage, KeyboardMessage, MainParameterMessage, MaskChangeMessage, MaskMessage, Message, MiscMessage, MouseMessage, RenderMessage, TaskMessage, ToolboxMessage, TopPaneMessage, WelcomeMessage}, workspace::parameters::CropPreset};

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

impl From<MouseEvent> for UpdateEvent {
    fn from(event: MouseEvent) -> Self {
        WorkspaceEvent::ImageMouseEvent(event).into()
    }
}

#[derive(Debug, Clone)]
pub enum WorkspaceEvent {
    ToggleCropMode,
    ToggleMaskMode(usize),
    BrightnessChanged(f32),
    ContrastChanged(f32),
    ShadowsChanged(f32),
    MidtonesChanged(f32),
    HighlightsChanged(f32),
    TintChanged(f32),
    TemperatureChanged(f32),
    SaturationChanged(f32),
    AddMask,
    DeleteMask(usize),
    MaskToggleLinear(usize, bool),
    MaskBrightnessChanged(usize, f32),
    MaskAngleChanged(usize, f32),
    MaskFeatherChanged(usize, f32),
    AngleChanged(f32),
    CropScaleChanged(f32),
    CropRotateLeft,
    CropRotateRight,
    CropPresetChanged(CropPreset),
    ToggleParametersVisibility,
    ToggleFavorite,
    ExportImage,
    Undo,
    Redo,
    ResetView,
    ToggleDisplayGrid,
    ImageMouseEvent(MouseEvent),
}

impl From<WorkspaceEvent> for UpdateEvent {
    fn from(event: WorkspaceEvent) -> UpdateEvent {
        UpdateEvent::WorkspaceEvent(event)
    }
}

#[derive(Debug, Clone)]
pub enum AlbumEvent {
    NextImage,
    PreviousImage,
    SetImage(usize)
}

impl From<AlbumEvent> for UpdateEvent {
    fn from(event: AlbumEvent) -> Self {
        UpdateEvent::AlbumEvent(event)
    }
}

#[derive(Debug, Clone)]
pub enum ImageManagerEvent {
    AddImages,
    Save,
    DeleteImage(i32),
    LoadImage(i32, RawImage, RawImage),
    ToggleFilter
}

impl From<ImageManagerEvent> for UpdateEvent {
    fn from(event: ImageManagerEvent) -> Self {
        UpdateEvent::ImageManagerEvent(event)
    }
}

pub enum UpdateEvent {
    OnStart,
    OnExit(iced::window::Id),
    WorkspaceEvent(WorkspaceEvent),
    AlbumEvent(AlbumEvent),
    ImageManagerEvent(ImageManagerEvent)
}

impl From<BottomPaneMessage> for UpdateEvent {
    fn from(message: BottomPaneMessage) -> Self {
        match message {
            BottomPaneMessage::NextImage => AlbumEvent::NextImage.into(),
            BottomPaneMessage::PreviousImage => AlbumEvent::PreviousImage.into(),
            BottomPaneMessage::ToggleFavorite => WorkspaceEvent::ToggleFavorite.into(),
            BottomPaneMessage::DeleteImage(photo_id) => ImageManagerEvent::DeleteImage(photo_id).into(),
            BottomPaneMessage::ResetView => WorkspaceEvent::ResetView.into(),
            BottomPaneMessage::ToggleDisplayGrid => WorkspaceEvent::ToggleDisplayGrid.into(),
            BottomPaneMessage::ToggleParametersVisibility => WorkspaceEvent::ToggleParametersVisibility.into()
        }
    }
}

impl From<MainParameterMessage> for UpdateEvent {
    fn from(message: MainParameterMessage) -> Self {
        match message {
            MainParameterMessage::BrightnessChanged(brightness) => WorkspaceEvent::BrightnessChanged(brightness).into(),
            MainParameterMessage::ContrastChanged(contrast) => WorkspaceEvent::ContrastChanged(contrast).into(),
            MainParameterMessage::ShadowsChanged(shadows) => WorkspaceEvent::ShadowsChanged(shadows).into(),
            MainParameterMessage::MidtonesChanged(midtones) => WorkspaceEvent::MidtonesChanged(midtones).into(),
            MainParameterMessage::HighlightsChanged(highlights) => WorkspaceEvent::HighlightsChanged(highlights).into(),
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
                    MaskChangeMessage::FeatherChanged(angle) => WorkspaceEvent::MaskFeatherChanged(mask_index, angle).into(),
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
            MiscMessage::CropScaleChanged(scale) => WorkspaceEvent::CropScaleChanged(scale).into(),
            MiscMessage::ToggleCropMode => WorkspaceEvent::ToggleCropMode.into(),
            MiscMessage::CropRotateLeft => WorkspaceEvent::CropRotateLeft.into(),
            MiscMessage::CropRotateRight => WorkspaceEvent::CropRotateRight.into(),
            MiscMessage::CropPresetChanged(crop_preset) => WorkspaceEvent::CropPresetChanged(crop_preset).into()
        }
    }
}

impl From<ToolboxMessage> for UpdateEvent {
    fn from(message: ToolboxMessage) -> Self {
        match message {
            ToolboxMessage::MainParameterMessage(message) => message.into(),
            ToolboxMessage::MaskMessage(message) => message.into(),
            ToolboxMessage::MiscMessage(message) => message.into(),
        }
    }
}

impl From<TopPaneMessage> for UpdateEvent {
    fn from(message: TopPaneMessage) -> Self {
        match message {
            TopPaneMessage::AddImages => ImageManagerEvent::AddImages.into(),
            TopPaneMessage::Export => WorkspaceEvent::ExportImage.into(),
            TopPaneMessage::Undo => WorkspaceEvent::Undo.into(),
            TopPaneMessage::Redo => WorkspaceEvent::Redo.into(),
            TopPaneMessage::ToggleFilter => ImageManagerEvent::ToggleFilter.into()
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
            RenderMessage::MouseMessage(message) => message.into()
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
            WelcomeMessage::AddImages => ImageManagerEvent::AddImages.into()
        }
    }
}

impl From<KeyboardMessage> for UpdateEvent {
    fn from(message: KeyboardMessage) -> Self {
        match message {
            KeyboardMessage::NextImage => AlbumEvent::NextImage.into(),
            KeyboardMessage::PreviousImage => AlbumEvent::PreviousImage.into(),
            KeyboardMessage::CropRotateLeft => WorkspaceEvent::CropRotateLeft.into(),
            KeyboardMessage::CropRotateRight => WorkspaceEvent::CropRotateRight.into(),
            KeyboardMessage::ToggleFavorite => WorkspaceEvent::ToggleFavorite.into(),
            KeyboardMessage::ToggleDisplayGrid => WorkspaceEvent::ToggleDisplayGrid.into(),
            KeyboardMessage::ToggleCropMode => WorkspaceEvent::ToggleCropMode.into()
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
                ImageManagerEvent::LoadImage(photo_id, image, thumbnail).into()
            }
        }
    }
}

impl From<Message> for UpdateEvent {
    fn from(message: Message) -> Self {
        match message {
            Message::OnStartMessage => UpdateEvent::OnStart,
            Message::OnWindowCloseMessage(window_id) => UpdateEvent::OnExit(window_id),
            Message::OnTimeTickMessage => ImageManagerEvent::Save.into(),
            Message::BottomPaneMessage(message) => message.into(),
            Message::ImageSelectionMessage(message) => message.into(),
            Message::RenderMessage(message) => message.into(),
            Message::ToolboxMessage(message) => message.into(),
            Message::TopPaneMessage(message) => message.into(),
            Message::WelcomeMessage(message) => message.into(),
            Message::KeyboardMessage(message) => message.into(),
            Message::TaskMessage(message) => message.into()
        }
    }
}