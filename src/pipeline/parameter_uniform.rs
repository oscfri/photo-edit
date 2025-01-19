#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct ParameterUniform {
    brightness: f32,
}

impl ParameterUniform {
    pub fn new() -> Self {
        Self {
            brightness: 0.1
        }
    }
}