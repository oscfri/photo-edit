use crate::workspace::parameters::Parameters;

#[derive(Default, Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct RadialParameter {
    center_x: f32,
    center_y: f32,
    width: f32,
    height: f32,
    angle: f32,
    feather: f32,
    brightness: f32,
    _1: f32,
}

#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct RadialParameters {
    entries: [RadialParameter; 128],
    count: u32
}

impl RadialParameters {
    pub fn new(parameters: &Parameters) -> RadialParameters {
        let mut entries = [RadialParameter::default(); 128];
        for (index, radial_mask) in parameters.radial_masks.iter().take(entries.len()).enumerate() {
            entries[index].center_x = radial_mask.center_x as f32;
            entries[index].center_y = radial_mask.center_y as f32;
            entries[index].width = radial_mask.width as f32;
            entries[index].height = radial_mask.height as f32;
            entries[index].angle = radial_mask.angle / 180.0 * std::f32::consts::PI;
            entries[index].brightness = radial_mask.brightness;
        }
        RadialParameters {
            entries,
            count: parameters.radial_masks.len() as u32
        }
    }
}