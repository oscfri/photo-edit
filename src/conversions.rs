use crate::types::*;

use rayon::prelude::*;

/**
 * Conversions based on: https://www.easyrgb.com/en/math.php
 */

const REFERENCE_X: f32 = 95.047;
const REFERENCE_Y: f32 = 100.0;
const REFERENCE_Z: f32 = 108.883;

const CACHE_PRECISION: i32 = 100;

 // TODO: Figure out how we can do this without cloning
 // TODO: Verify if this actually makes things faster or not...
 #[derive(Clone)]
pub struct Converter {
    pow_rgb_to_xyz: Vec<f32>,
    pow_xyz_to_lab: Vec<f32>,
    pow_lab_to_xyz: Vec<f32>,
    pow_xyz_to_rgb: Vec<f32>
}

pub fn create_converter() -> Converter {
    // TODO: There might be some inaccuracies with the inverse pow formulas.
    // rgb -> xyz -> rgb probably doesn't yield same result
    Converter {
        pow_rgb_to_xyz: precompute_powf(2.4),
        pow_xyz_to_lab: precompute_powf(1.0 / 3.0),
        pow_lab_to_xyz: precompute_powf(3.0),
        pow_xyz_to_rgb: precompute_powf(1.0 / 2.4)
    }
}

fn precompute_powf(exponent: f32) -> Vec<f32> {
    let scaling = 1.0 / (CACHE_PRECISION as f32);
    (0..100 * CACHE_PRECISION)
        .map(|x: i32| (x as f32) * scaling)
        .map(|x: f32| x.powf(exponent))
        .collect()
}

fn cached_powf(value: f32, pow: &Vec<f32>) -> f32 {
    let scaled_value: f32 = value * (CACHE_PRECISION as f32);
    let index = scaled_value as usize;
    let alpha: f32 = scaled_value - (index as f32);
    if index < pow.len() - 1 {
        alpha * pow[index + 1] + (1.0 - alpha) * pow[index]
    } else {
        // TODO: This shouldn't happen, so better think about what's best here
        value
    }
}

impl Converter {
    pub fn rgb_image_to_lab(&self, rgb_image: &RgbImage) -> LabImage {
        let lab_pixels: Vec<LabPixel> = rgb_image.pixels.par_iter()
            .map(|rgb_pixel: &RgbPixel| self.rgb_pixel_to_lab(rgb_pixel))
            .collect();

        LabImage {
            width: rgb_image.width,
            height: rgb_image.height,
            pixels: lab_pixels
        }
    }

    pub fn lab_image_to_rgb(&self, lab_image: &LabImage) -> RgbImage {
        let rgb_pixels: Vec<RgbPixel> = lab_image.pixels.par_iter()
            .map(|lab_pixel: &LabPixel| self.lab_pixel_to_rgb(lab_pixel))
            .collect();

        RgbImage {
            width: lab_image.width,
            height: lab_image.height,
            pixels: rgb_pixels
        }
    }

    fn rgb_pixel_to_lab(&self, rgb_pixel: &RgbPixel) -> LabPixel {
        let xyz_pixel: XyzPixel = self.rgb_pixel_to_xyz(&rgb_pixel);
        let lab_pixel: LabPixel = self.xyz_pixel_to_lab(&xyz_pixel);
        lab_pixel
    }

    fn lab_pixel_to_rgb(&self, lab_pixel: &LabPixel) -> RgbPixel {
        if lab_pixel.lightness < 0.01 {
            RgbPixel {
                red: 0.0,
                green: 0.0,
                blue: 0.0
            }
        } else {
            let xyz_pixel: XyzPixel = self.lab_pixel_to_xyz(&lab_pixel);
            let rgb_pixel: RgbPixel = self.xyz_pixel_to_rgb(&xyz_pixel);
            rgb_pixel
        }
    }

    fn scale_rgb_to_xyz(&self, value: f32) -> f32 {
        if value > 0.04045 {
            100.0 * cached_powf((value + 0.055) / 1.055, &self.pow_rgb_to_xyz)
        } else {
            100.0 * value / 12.92
        }
    }

    fn rgb_pixel_to_xyz(&self, rgb_pixel: &RgbPixel) -> XyzPixel {
        let scaled_red: f32 = self.scale_rgb_to_xyz(rgb_pixel.red);
        let scaled_green: f32 = self.scale_rgb_to_xyz(rgb_pixel.green);
        let scaled_blue: f32 = self.scale_rgb_to_xyz(rgb_pixel.blue);

        XyzPixel {
            x: scaled_red * 0.4124 + scaled_green * 0.3576 + scaled_blue * 0.1805,
            y: scaled_red * 0.2126 + scaled_green * 0.7152 + scaled_blue * 0.0822,
            z: scaled_red * 0.0193 + scaled_green * 0.1192 + scaled_blue * 0.9505
        }
    }


    fn scale_xyz_to_lab(&self, value: f32) -> f32 {
        if value > 0.008856 {
            cached_powf(value, &self.pow_xyz_to_lab)
        } else {
            (7.787 * value) + (16.0 / 116.0)
        }
    }

    fn xyz_pixel_to_lab(&self, xyz_pixel: &XyzPixel) -> LabPixel {
        let var_x: f32 = self.scale_xyz_to_lab(xyz_pixel.x / REFERENCE_X);
        let var_y: f32 = self.scale_xyz_to_lab(xyz_pixel.y / REFERENCE_Y);
        let var_z: f32 = self.scale_xyz_to_lab(xyz_pixel.z / REFERENCE_Z);

        LabPixel {
            lightness: (116.0 * var_y) - 16.0,
            tint: 500.0 * (var_x - var_y),
            temperature: 200.0 * (var_y - var_z)
        }
    }


    fn scale_lab_to_xyz(&self, value: f32) -> f32 {
        if value > 0.008856 {
            cached_powf(value, &self.pow_lab_to_xyz)
        } else {
            (value - 16.0 / 116.0) / 7.787
        }
    }

    fn lab_pixel_to_xyz(&self, lab_pixel: &LabPixel) -> XyzPixel {
        let var_y: f32 = (lab_pixel.lightness + 16.0) / 116.0;
        let var_x: f32 = (lab_pixel.tint / 500.0) + var_y;
        let var_z: f32 = var_y - (lab_pixel.temperature / 200.0);

        XyzPixel {
            x: self.scale_lab_to_xyz(var_x) * REFERENCE_X,
            y: self.scale_lab_to_xyz(var_y) * REFERENCE_Y,
            z: self.scale_lab_to_xyz(var_z) * REFERENCE_Z
        }
    }


    fn scale_xyz_to_rgb(&self, value: f32) -> f32 {
        let scaled_value: f32 = value / 100.0;
        if scaled_value > 0.0031308 {
            1.055 * cached_powf(scaled_value, &self.pow_xyz_to_rgb) - 0.055
        } else {
            scaled_value * 12.92
        }
    }

    fn xyz_pixel_to_rgb(&self, xyz_pixel: &XyzPixel) -> RgbPixel {
        RgbPixel {
            red:   self.scale_xyz_to_rgb( xyz_pixel.x * 3.2406 - xyz_pixel.y * 1.5372 - xyz_pixel.z * 0.4986),
            green: self.scale_xyz_to_rgb(-xyz_pixel.x * 0.9689 + xyz_pixel.y * 1.8758 + xyz_pixel.z * 0.0415),
            blue:  self.scale_xyz_to_rgb( xyz_pixel.x * 0.0557 - xyz_pixel.y * 0.2040 + xyz_pixel.z * 1.0570)
        }
    }
}