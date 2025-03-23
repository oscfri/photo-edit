use super::{transform::Rectangle, viewport::ViewportWorkspace};

#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct CropUniform {
    width: f32,
    height: f32,
    ratio: f32,
    display_grid: i32,
}

impl CropUniform {
    pub fn new(workspace: &ViewportWorkspace, bounds: &Rectangle, scale_factor: f32) -> Self {
        let ratio;
        if bounds.width < bounds.height {
            ratio = bounds.width / (workspace.view.width as f32) * scale_factor;
        } else {
            ratio = bounds.height / (workspace.view.height as f32) * scale_factor;
        }
        Self {
            width: workspace.parameters.crop.width as f32,
            height: workspace.parameters.crop.height as f32,
            ratio,
            display_grid: workspace.display_grid.into()
        }
    }
}