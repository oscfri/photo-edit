#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ViewMode {
    Normal,
    Crop,
    Mask(usize)
}