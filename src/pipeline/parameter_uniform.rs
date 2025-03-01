use crate::workspace::parameters::Parameters;

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
            contrast: (parameters.contrast * 0.01 + 100.0) / 100.0,
            tint: parameters.tint * 0.001,
            temperature: parameters.temperature * 0.001,
            saturation: (parameters.saturation + 100.0) / 100.0
        }
    }
}