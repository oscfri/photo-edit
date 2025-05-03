use std::time::SystemTime;

use serde;

#[derive(Debug, Default, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Parameters {
    pub brightness: f32, // TODO: Rename to exposure
    pub contrast: f32,
    pub tint: f32,
    pub temperature: f32,
    pub saturation: f32,
    pub radial_masks: Vec<RadialMask>,
    pub crop: Option<Crop>,
    pub is_favorite: bool,
}

#[derive(Debug, Default, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RadialMask {
    pub center_x: i32,
    pub center_y: i32,
    pub width: i32,
    pub height: i32,
    pub angle: f32,
    pub feather: f32,
    pub brightness: f32,
    pub is_linear: bool
}

#[derive(Debug, Default, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Crop {
    pub center_x: i32,
    pub center_y: i32,
    pub source_image_width: usize,
    pub source_image_height: usize,
    pub scale: f32,
    pub angle_degrees: f32,
    pub preset: CropPreset,
    pub rotation: i32,
}

impl Crop {
    pub fn get_full_angle(&self) -> f32 {
        self.angle_degrees + (self.rotation as f32) * 90.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum CropPreset {
    Original,
    Ratio(i32, i32)
}

impl Default for CropPreset {
    fn default() -> Self {
        Self::Original
    }
}

impl std::fmt::Display for CropPreset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Original => write!(f, "Original"),
            Self::Ratio(width, height) => write!(f, "{}:{}", width, height)
        }
    }
}

impl CropPreset {
    pub fn rotate(&self) -> CropPreset {
        match self {
            Self::Original => Self::Original,
            Self::Ratio(width, height) => Self::Ratio(*height, *width)
        }
    }
}

pub struct ParameterHistory {
    parameters: Parameters,
    parameter_history: Vec<Parameters>,
    parameter_history_index: usize,
    last_updated: SystemTime,
}

impl From<Parameters> for ParameterHistory {
    fn from(parameters: Parameters) -> Self {
        let parameter_history = vec![parameters.clone()];
        let parameter_history_index = 0;
        let last_updated = SystemTime::now();
        Self {
            parameters,
            parameter_history,
            parameter_history_index,
            last_updated
        }
    }
}

impl ParameterHistory {
    pub fn update<F>(&mut self, function: F) where F: FnOnce(&mut Parameters) {
        function(&mut self.parameters);

        if self.has_changed() {
            if self.needs_new() {
                self.parameter_history.push(self.parameters.clone());
                self.parameter_history_index += 1;
                self.last_updated = SystemTime::now();
            } else {
                self.parameter_history[self.parameter_history_index] = self.parameters.clone();
            }
        }
    }

    pub fn current(&self) -> Parameters {
        self.parameters.clone()
    }

    pub fn undo(&mut self) {
        if self.parameter_history_index > 0 {
            self.parameter_history_index -= 1;
            self.parameters = self.parameter_history[self.parameter_history_index].clone()
        }
    }

    pub fn redo(&mut self) {
        if self.parameter_history_index < self.parameter_history.len() - 1 {
            self.parameter_history_index += 1;
            self.parameters = self.parameter_history[self.parameter_history_index].clone()
        }
    }

    fn needs_new(&self) -> bool {
        if let Ok(elapsed) = self.last_updated.elapsed() {
            elapsed.as_secs() >= 1
        } else {
            false
        }
    }

    fn has_changed(&self) -> bool {
        !self.parameter_history[self.parameter_history_index].eq(&self.parameters)
    }
}