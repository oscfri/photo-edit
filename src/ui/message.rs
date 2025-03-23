use crate::workspace::{image_loader::ImageLoadResult, parameters::CropPreset};

#[derive(Debug, Clone, Copy)]
pub enum MouseState {
    Up,
    Down
}

#[derive(Debug, Clone)]
pub enum BottomPaneMessage {
    NextImage,
    PreviousImage,
    ToggleFavorite,
    DeleteImage(i32),
    ToggleParametersVisibility
}

#[derive(Debug, Clone)]
pub enum MouseMessage {
    Over,
    Press,
    RightPress,
    Release,
    Scroll(f32),
}

#[derive(Debug, Clone)]
pub enum MainParameterMessage {
    BrightnessChanged(f32),
    ContrastChanged(f32),
    SaturationChanged(f32),
    TemperatureChanged(f32),
    TintChanged(f32),
}

#[derive(Debug, Clone)]
pub enum MaskChangeMessage {
    DeleteMask,
    ToggleMaskMode,
    MaskToggleLinear(bool),
    BrightnessChanged(f32),
    MaskAngleChanged(f32),
    FeatherChanged(f32)
}

#[derive(Debug, Clone)]
pub enum MaskMessage {
    AddMask,
    MaskChanged(usize, MaskChangeMessage)
}

#[derive(Debug, Clone)]
pub enum MiscMessage {
    AngleChanged(f32),
    ToggleCropMode,
    CropRotateLeft,
    CropRotateRight,
    CropPresetChanged(CropPreset)
}

#[derive(Debug, Clone)]
pub enum ToolboxMessage {
    MainParameterMessage(MainParameterMessage),
    MaskMessage(MaskMessage),
    MiscMessage(MiscMessage)
}

#[derive(Debug, Clone)]
pub enum TopPaneMessage {
    AddImages,
    Export,
    Undo,
    Redo,
    ToggleFilter
}

#[derive(Debug, Clone)]
pub enum RenderMessage {
    MouseMessage(MouseMessage)
}

#[derive(Debug, Clone)]
pub enum ImageSelectionMessage {
    SelectImage(usize)
}

#[derive(Debug, Clone)]
pub enum WelcomeMessage {
    AddImages
}

#[derive(Debug, Clone)]
pub enum TaskMessage {
    NewImage(ImageLoadResult)
}

#[derive(Debug, Clone)]
pub enum Message {
    OnStartMessage,
    OnWindowCloseMessage(iced::window::Id),
    OnTimeTickMessage,

    BottomPaneMessage(BottomPaneMessage),
    ImageSelectionMessage(ImageSelectionMessage),
    RenderMessage(RenderMessage),
    ToolboxMessage(ToolboxMessage),
    TopPaneMessage(TopPaneMessage),
    WelcomeMessage(WelcomeMessage),

    TaskMessage(TaskMessage)
}