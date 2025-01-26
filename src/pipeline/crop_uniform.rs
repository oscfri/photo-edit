use crate::{album::Crop, view_mode};

#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct CropUniform {
    top_left: glam::IVec2,
    bottom_right: glam::IVec2,
    image_size: glam::Vec2,
    visible: i32
}

impl CropUniform {
    pub fn new(crop: &Crop, view_mode: &view_mode::ViewMode, image_width: usize, image_height: usize) -> Self {
        let min_x: i32 = std::cmp::min(crop.x1, crop.x2);
        let max_x: i32 = std::cmp::max(crop.x1, crop.x2);
        let min_y: i32 = std::cmp::min(crop.y1, crop.y2);
        let max_y: i32 = std::cmp::max(crop.y1, crop.y2);
        Self {
            top_left: glam::ivec2(min_x, min_y),
            bottom_right: glam::ivec2(max_x, max_y),
            image_size: glam::vec2(image_width as f32, image_height as f32),
            visible: matches!(view_mode, view_mode::ViewMode::Crop) as i32
        }
    }
}