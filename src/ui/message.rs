#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum Message {
    LoadAlbum,
    SaveAlbum,
    ExportImage,
    NextImage,
    DeleteImage,
    SetImage(usize),
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
    ImageMouseMessage(MouseMessage),
}