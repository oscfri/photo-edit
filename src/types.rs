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

/**
 * Conversions based on: https://www.easyrgb.com/en/math.php
 */

const REFERENCE_X: f32 = 95.047;
const REFERENCE_Y: f32 = 100.0;
const REFERENCE_Z: f32 = 108.883;

pub fn rgb_pixel_to_lab(rgb_pixel: RgbPixel) -> LabPixel {
    let xyz_pixel: XyzPixel = rgb_pixel_to_xyz(&rgb_pixel);
    let lab_pixel: LabPixel = xyz_pixel_to_lab(&xyz_pixel);
    lab_pixel
}

fn scale_rgb_to_xyz(value: f32) -> f32 {
    if value > 0.04045 {
        100.0 * ((value + 0.055) / 1.055).powf(2.4)
    } else {
        100.0 * value / 12.92
    }
}

fn rgb_pixel_to_xyz(rgb_pixel: &RgbPixel) -> XyzPixel {
    let scaled_red: f32 = scale_rgb_to_xyz(rgb_pixel.red);
    let scaled_green: f32 = scale_rgb_to_xyz(rgb_pixel.green);
    let scaled_blue: f32 = scale_rgb_to_xyz(rgb_pixel.blue);

    XyzPixel {
        x: scaled_red * 0.4124 + scaled_green * 0.3576 + scaled_blue * 0.1805,
        y: scaled_red * 0.2126 + scaled_green * 0.7152 + scaled_blue * 0.0822,
        z: scaled_red * 0.0193 + scaled_green * 0.1192 + scaled_blue * 0.9505
    }
}

fn scale_xyz_to_lab(value: f32) -> f32 {
    if value > 0.008856 {
        value.powf(1.0 / 3.0)
    } else {
        (7.787 * value) + (16.0 / 116.0)
    }
}

fn xyz_pixel_to_lab(xyz_pixel: &XyzPixel) -> LabPixel {
    let var_x: f32 = scale_xyz_to_lab(xyz_pixel.x / REFERENCE_X);
    let var_y: f32 = scale_xyz_to_lab(xyz_pixel.y / REFERENCE_Y);
    let var_z: f32 = scale_xyz_to_lab(xyz_pixel.z / REFERENCE_Z);

    LabPixel {
        lightness: (116.0 * var_y) - 16.0,
        tint: 500.0 * (var_x - var_y),
        temperature: 200.0 * (var_y - var_z)
    }
}