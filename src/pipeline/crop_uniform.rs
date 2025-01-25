use crate::album::Crop;

#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct CropUniform {
    top_left: glam::Vec2,
    bottom_right: glam::Vec2
}

impl CropUniform {
    pub fn new(crop: &Crop) -> Self {
        let min_x: f32 = std::cmp::min(crop.x1, crop.x2) as f32;
        let max_x: f32 = std::cmp::max(crop.x1, crop.x2) as f32;
        let min_y: f32 = std::cmp::min(crop.y1, crop.y2) as f32;
        let max_y: f32 = std::cmp::max(crop.y1, crop.y2) as f32;
        Self {
            top_left: glam::vec2(min_x, min_y),
            bottom_right: glam::vec2(max_x, max_y),
        }
    }
}