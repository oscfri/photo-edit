use crate::types::*;

use rayon::prelude::*;

/**
 * Conversions based on: https://www.easyrgb.com/en/math.php
 */

const REFERENCE_X: f32 = 95.047;
const REFERENCE_Y: f32 = 100.0;
const REFERENCE_Z: f32 = 108.883;

pub fn rgb_image_to_lab(rgb_image: &RgbImage) -> LabImage {
    let lab_pixels: Vec<LabPixel> = rgb_image.pixels.par_iter()
        .map(|rgb_pixel: &RgbPixel| {
            let xyz_pixel: XyzPixel = rgb_pixel_to_xyz(&rgb_pixel);
            let lab_pixel: LabPixel = xyz_pixel_to_lab(&xyz_pixel);
            lab_pixel
        })
        .collect();

    LabImage {
        width: rgb_image.width,
        height: rgb_image.height,
        pixels: lab_pixels
    }
}

pub fn lab_image_to_rgb(lab_image: &LabImage) -> RgbImage {
    let rgb_pixels: Vec<RgbPixel> = lab_image.pixels.par_iter()
        .map(|lab_pixel: &LabPixel| {
            let xyz_pixel: XyzPixel = lab_pixel_to_xyz(&lab_pixel);
            let rgb_pixel: RgbPixel = xyz_pixel_to_rgb(&xyz_pixel);
            rgb_pixel
        })
        .collect();

    RgbImage {
        width: lab_image.width,
        height: lab_image.height,
        pixels: rgb_pixels
    }
}

fn rgb_pixel_to_xyz(rgb_pixel: &RgbPixel) -> XyzPixel {
    fn scale(value: f32) -> f32 {
        if value > 0.04045 {
            100.0 * ((value + 0.055) / 1.055).powf(2.4)
        } else {
            100.0 * value / 12.92
        }
    }

    let scaled_red: f32 = scale(rgb_pixel.red);
    let scaled_green: f32 = scale(rgb_pixel.green);
    let scaled_blue: f32 = scale(rgb_pixel.blue);

    XyzPixel {
        x: scaled_red * 0.4124 + scaled_green * 0.3576 + scaled_blue * 0.1805,
        y: scaled_red * 0.2126 + scaled_green * 0.7152 + scaled_blue * 0.0822,
        z: scaled_red * 0.0193 + scaled_green * 0.1192 + scaled_blue * 0.9505
    }
}

fn xyz_pixel_to_lab(xyz_pixel: &XyzPixel) -> LabPixel {
    fn scale(value: f32) -> f32 {
        if value > 0.008856 {
            value.powf(1.0 / 3.0)
        } else {
            (7.787 * value) + (16.0 / 116.0)
        }
    }

    let var_x: f32 = scale(xyz_pixel.x / REFERENCE_X);
    let var_y: f32 = scale(xyz_pixel.y / REFERENCE_Y);
    let var_z: f32 = scale(xyz_pixel.z / REFERENCE_Z);

    LabPixel {
        lightness: (116.0 * var_y) - 16.0,
        tint: 500.0 * (var_x - var_y),
        temperature: 200.0 * (var_y - var_z)
    }
}

fn lab_pixel_to_xyz(lab_pixel: &LabPixel) -> XyzPixel {
    fn scale(value: f32) -> f32 {
        if value > 0.008856 {
            value.powf(3.0)
        } else {
            (value - 16.0 / 116.0) / 7.787
        }
    }

    let var_y: f32 = (lab_pixel.lightness + 16.0) / 116.0;
    let var_x: f32 = (lab_pixel.tint / 500.0) + var_y;
    let var_z: f32 = var_y - (lab_pixel.temperature / 200.0);

    XyzPixel {
        x: scale(var_x) * REFERENCE_X,
        y: scale(var_y) * REFERENCE_Y,
        z: scale(var_z) * REFERENCE_Z
    }
}

fn xyz_pixel_to_rgb(xyz_pixel: &XyzPixel) -> RgbPixel {
    fn scale(value: f32) -> f32 {
        let scaled_value: f32 = value / 100.0;
        if scaled_value > 0.0031308 {
            1.055 * scaled_value.powf(1.0 / 2.4) - 0.055
        } else {
            scaled_value * 12.92
        }
    }

    RgbPixel {
        red:   scale( xyz_pixel.x * 3.2406 - xyz_pixel.y * 1.5372 - xyz_pixel.z * 0.4986),
        green: scale(-xyz_pixel.x * 0.9689 + xyz_pixel.y * 1.8758 + xyz_pixel.z * 0.0415),
        blue:  scale( xyz_pixel.x * 0.0557 - xyz_pixel.y * 0.2040 + xyz_pixel.z * 1.0570)
    }
}