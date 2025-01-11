#[derive(Clone)]
pub struct RgbPixel {
    pub red: f32,
    pub green: f32,
    pub blue: f32
}

#[derive(Clone)]
pub struct XyzPixel {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

#[derive(Clone)]
pub struct LabPixel {
    pub lightness: f32, // L*
    pub tint: f32, // a*
    pub temperature: f32 // b*
}

#[derive(Clone)]
pub struct RgbImage {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<RgbPixel>
}

#[derive(Clone)]
pub struct LabImage {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<LabPixel>
}

#[derive(Clone, Debug)]
pub struct RawImage {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<u8>
}