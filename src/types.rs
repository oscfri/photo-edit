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
 * XYZ -> Oklab conversions based on: https://bottosson.github.io/posts/oklab/
 */

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

fn xyz_pixel_to_lab(xyz_pixel: &XyzPixel) -> LabPixel {
    let mut l: f32 = 0.4122214708 * xyz_pixel.x + 0.5363325363 * xyz_pixel.y + 0.0514459929 * xyz_pixel.z;
    let mut m: f32 = 0.2119034982 * xyz_pixel.x + 0.6806995451 * xyz_pixel.y + 0.1073969566 * xyz_pixel.z;
    let mut s: f32 = 0.0883024619 * xyz_pixel.x + 0.2817188376 * xyz_pixel.y + 0.6299787005 * xyz_pixel.z;
    
    l = l.powf(1.0 / 3.0);
    m = m.powf(1.0 / 3.0);
    s = s.powf(1.0 / 3.0);
    
    LabPixel {
        lightness:   0.2104542553 * l + 0.7936177850 * m - 0.0040720468 * s,
        tint:        1.9779984951 * l - 2.4285922050 * m + 0.4505937099 * s,
        temperature: 0.0259040371 * l + 0.7827717662 * m - 0.8086757660 * s,
    }
}