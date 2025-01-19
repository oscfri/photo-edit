use crate::album::Parameters;

#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct ParameterUniform {
    brightness: f32,
}

impl ParameterUniform {
    pub fn new(parameters: &Parameters) -> Self {
        Self {
            brightness: parameters.brightness * 0.01
        }
    }
}