#[derive(Debug, Clone)]
pub enum ViewMode {
    Normal,
    Crop,
    Mask(usize)
}