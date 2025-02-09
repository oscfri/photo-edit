use crate::view_mode;

#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct CropUniform {
    visible: i32
}

impl CropUniform {
    pub fn new(view_mode: &view_mode::ViewMode) -> Self {
        Self {
            visible: matches!(view_mode, view_mode::ViewMode::Crop) as i32
        }
    }
}