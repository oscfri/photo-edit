use glam;
use iced::widget::shader;

use crate::album::Crop;

#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct CameraUniform {
    camera_position: glam::Vec2,
    camera_size: glam::Vec2,
    view_position: glam::Vec2,
    view_size: glam::Vec2,
}

fn calculate_size(bounds: &iced::Rectangle, viewport: &shader::Viewport, aspect_ratio: f32) -> glam::Vec2 {
    let bounds_aspect_ratio: f32 = bounds.width / bounds.height;
    if aspect_ratio > bounds_aspect_ratio {
        glam::vec2(
            bounds.width / (viewport.physical_width() as f32),
            bounds.width / aspect_ratio / (viewport.physical_height() as f32)
        )
    } else {
        glam::vec2(
            bounds.height * aspect_ratio / (viewport.physical_width() as f32),
            bounds.height / (viewport.physical_height() as f32)
        )
    }
}

impl CameraUniform {
    pub fn new(bounds: &iced::Rectangle, viewport: &shader::Viewport, view: &Crop) -> Self {
        let bottom_y: f32 = bounds.y * 2.0 + bounds.height;
        // Don't know why this "Mystery" offset is needed. (Seems like it's related to the toolbar to the right)
        let mystery_offset: f32 = 300.0;
        let crop_width: i32 = (view.x2 - view.x1).abs();
        let crop_height: i32 = (view.y2 - view.y1).abs();
        let aspect_ratio: f32 = (crop_width as f32) / (crop_height as f32);
        let min_x: i32 = std::cmp::min(view.x1, view.x2);
        let min_y: i32 = std::cmp::min(view.y1, view.y2);
        Self {
            camera_position: glam::vec2(
                (bounds.x - mystery_offset) / (viewport.physical_width() as f32),
                1.0 - bottom_y / (viewport.physical_height() as f32)
            ),
            camera_size: calculate_size(bounds, viewport, aspect_ratio),
            view_position: glam::vec2(min_x as f32, min_y as f32),
            view_size: glam::vec2((view.x2 - view.x1).abs() as f32, (view.y2 - view.y1).abs() as f32),
        }
    }
}