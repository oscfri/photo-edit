use super::{transform::Rectangle, viewport::ViewportWorkspace};

#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct CropUniform {
    width: f32,
    height: f32,
    ratio: f32,
    _1: f32,
}

impl CropUniform {
    pub fn new(workspace: &ViewportWorkspace, viewport: &Rectangle) -> Self {
        let ratio = viewport.width / (workspace.get_image_width() as f32);
        Self {
            width: workspace.crop.width as f32,
            height: workspace.crop.height as f32,
            ratio: ratio,
            _1: 0.0
        }
    }
}