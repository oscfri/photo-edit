use super::viewport::ViewportParameters;

#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct ParameterUniform {
    brightness: f32,
    contrast: f32,
    shadows: f32,
    highlights: f32,
    tint: f32,
    temperature: f32,
    saturation: f32,
}

impl ParameterUniform {
    pub fn new(parameters: &ViewportParameters) -> Self {
        Self {
            brightness: parameters.brightness,
            contrast: (parameters.contrast * 0.5 + 100.0) / 100.0,
            shadows: parameters.shadows,
            highlights: parameters.highlights,
            tint: parameters.tint * 0.001,
            temperature: parameters.temperature * 0.001,
            saturation: (parameters.saturation + 100.0) / 100.0
        }
    }
}