use crate::album::Parameters;

#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct ParameterUniform {
    brightness: f32,
    contrast: f32,
    tint: f32,
    temperature: f32,
    saturation: f32,
}

impl ParameterUniform {
    pub fn new(parameters: &Parameters) -> Self {
        Self {
            brightness: parameters.brightness,
            contrast: parameters.contrast,
            tint: parameters.tint,
            temperature: parameters.temperature,
            saturation: parameters.saturation
        }
    }
}