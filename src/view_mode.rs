#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ViewMode {
    Normal,
    Crop,
    Mask(usize)
}

impl ViewMode {
    pub fn toggle_view_mode(&self, view_mode: ViewMode) -> ViewMode {
        if *self == view_mode {
            ViewMode::Normal
        } else {
            view_mode
        }
    }
}