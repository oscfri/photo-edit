use std::path::PathBuf;

use rayon::prelude::*;

use crate::types::{RawImage, RgbImage, RgbPixel};

#[derive(Clone, Debug)]
pub struct ImageLoadResult {
    pub photo_id: i32,
    pub image: RawImage,
    pub thumbnail: RawImage
}

pub async fn load_image(photo_id: i32, path: PathBuf) -> ImageLoadResult {
    let source_image = image::open(path).unwrap().into_rgb32f();
    let width: u32 = source_image.width();
    let height: u32 = source_image.height();
    let size = width * height;
    let mut pixels: Vec<RgbPixel> = Vec::with_capacity(size as usize);
    for h in 0..height {
        for w in 0..width {
            let rgb = source_image.get_pixel(w, h);
            pixels.push(RgbPixel {
                red: rgb[0],
                green: rgb[1],
                blue: rgb[2]
            });
        }
    }
    let rgb_image = RgbImage {
        width: width as usize,
        height: height as usize,
        pixels: pixels
    };
    let image = convert_to_raw_image(&rgb_image);
    let thumbnail = convert_to_raw_image(&resize_to_thumbnail_size(&rgb_image));
    ImageLoadResult { photo_id, image, thumbnail }
}

fn convert_to_raw_image(image: &RgbImage) -> RawImage {
    let mut buffer: Vec<u8> = vec![255; image.width * image.height * 4];

    buffer.par_iter_mut()
        .enumerate()
        .for_each(|(index, byte)| {
            let pixel_index: usize = index / 4;
            let channel_index: usize = index % 4;
            if channel_index == 0 {
                *byte = pixel_value_to_u8(image.pixels[pixel_index].red);
            } else if channel_index == 1 {
                *byte = pixel_value_to_u8(image.pixels[pixel_index].green);
            } else if channel_index == 2 {
                *byte = pixel_value_to_u8(image.pixels[pixel_index].blue);
            }
            // Don't bother with alpha, as it's 255 by default
        });

    RawImage {
        width: image.width,
        height: image.height,
        pixels: buffer
    }
}

fn pixel_value_to_u8(value: f32) -> u8 {
    if value <= 0.0 {
        0
    } else if value >= 1.0 {
        255
    } else {
        (value * 255.0) as u8
    }
}

fn resize_to_thumbnail_size(image: &RgbImage) -> RgbImage {
    let target_size: usize = 100;
    let width_skip: usize = std::cmp::max(1, image.width / target_size);
    let height_skip: usize = std::cmp::max(1, image.height / target_size);

    let target_width: usize = image.width / width_skip;
    let target_height: usize = image.height / height_skip;
    let mut pixels: Vec<RgbPixel> = Vec::new();

    for h in 0..target_height {
        for w in 0..target_width {
            pixels.push(image.pixels[(h * height_skip) * image.width + w * width_skip].clone());
        }
    }

    RgbImage {
        width: target_width,
        height: target_height,
        pixels: pixels
    }
}