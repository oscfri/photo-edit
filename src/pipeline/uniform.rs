use glam;
use iced::widget::shader;

#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct Uniform {
    camera_position: glam::Vec2,
    camera_size: glam::Vec2,
}

impl Uniform {
    pub fn new(bounds: &iced::Rectangle, viewport: &shader::Viewport) -> Self {
        let bottom_y = bounds.y * 2.0 + bounds.height;
        Self {
            camera_position: glam::vec2(
                bounds.x / (viewport.physical_width() as f32),
                1.0 - bottom_y / (viewport.physical_height() as f32)
            ),
            camera_size: glam::vec2(        
                bounds.width / (viewport.physical_width() as f32),
                bounds.height / (viewport.physical_height() as f32)
            )
        }
    }
}