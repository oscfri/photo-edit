use serde;

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct Parameters {
    pub brightness: f32, // TODO: Rename to exposure
    pub contrast: f32,
    pub tint: f32,
    pub temperature: f32,
    pub saturation: f32,
    pub radial_masks: Vec<RadialMask>,
    pub crop: Option<Crop>
}

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct RadialMask {
    pub center_x: i32,
    pub center_y: i32,
    pub width: i32,
    pub height: i32,
    pub angle: f32,
    pub brightness: f32,
    pub is_linear: bool
}

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct Crop {
    pub center_x: i32,
    pub center_y: i32,
    pub width: i32,
    pub height: i32,
    pub angle_degrees: f32,
}