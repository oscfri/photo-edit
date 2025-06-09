use crate::view_mode::ViewMode;

use super::viewport::ViewportParameters;

#[derive(Default, Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct RadialParameter {
    center_x: f32,
    center_y: f32,
    width: f32,
    height: f32,
    angle: f32,
    feather: f32,
    exposure: f32,
    draw_boundary: u32,
}

#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct RadialParameters {
    entries: [RadialParameter; 128],
    count: u32
}

impl RadialParameters {
    pub fn new(parameters: &ViewportParameters, view_mode: ViewMode) -> RadialParameters {
        let mut entries = [RadialParameter::default(); 128];
        for (index, radial_mask) in parameters.radial_masks.iter().take(entries.len()).enumerate() {
            entries[index].center_x = radial_mask.center_x as f32;
            entries[index].center_y = radial_mask.center_y as f32;
            entries[index].width = radial_mask.width as f32;
            if radial_mask.is_linear {
                entries[index].height = f32::INFINITY;
            } else {
                entries[index].height = radial_mask.height as f32;
            }
            entries[index].angle = radial_mask.angle_degrees / 180.0 * std::f32::consts::PI;
            entries[index].feather = (radial_mask.feather + 100.0) / 200.0;
            entries[index].exposure = radial_mask.brightness;
            entries[index].draw_boundary = Self::should_draw_boundary(index, view_mode);
        }
        RadialParameters {
            entries,
            count: parameters.radial_masks.len() as u32
        }
    }

    fn should_draw_boundary(index: usize, view_mode: ViewMode) -> u32 {
        match view_mode {
            ViewMode::Mask(mask_index) => {
                if index == mask_index {
                    1
                } else {
                    0
                }
            },
            _ => 0
        }
    }
}