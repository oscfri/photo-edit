#[derive(Clone)]
pub struct RgbPixel {
    pub red: f32,
    pub green: f32,
    pub blue: f32
}

#[derive(Clone)]
pub struct RgbImage {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<RgbPixel>
}

#[derive(Clone, Debug)]
pub struct RawImage {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<u8>
}