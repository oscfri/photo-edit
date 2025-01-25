use std::path::PathBuf;

use crate::types::*;

use rayon::prelude::*;

pub struct Album {
    pub images: Vec<AlbumImage>
}

pub struct AlbumImage {
    pub source_image: RawImage,
    pub parameters: Parameters,
    pub crop: Crop,
    pub thumbnail: RawImage
}

impl AlbumImage {

    // TODO: Reimplement this
    // pub fn pixel_at(&self, x: usize, y: usize) -> Option<LabPixel> {
    //     if x < self.source_image.width && y < self.source_image.height {
    //         Some(self.source_image.pixels[y * self.source_image.width + x].clone())
    //     } else {
    //         None
    //     }
    // }
}

#[derive(Debug, Default, Clone)]
pub struct Parameters {
    pub brightness: f32,
    pub contrast: f32,
    pub tint: f32,
    pub temperature: f32,
    pub saturation: f32
}

#[derive(Debug, Default, Clone)]
pub struct Crop {
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32
}

pub fn load_album(file_paths: &Vec<PathBuf>) -> Album {
    let images: Vec<AlbumImage> = file_paths.iter().map(load_album_image).collect();
    Album {
        images: images
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

fn load_album_image(path: &PathBuf) -> AlbumImage {
    let rgb_image: RgbImage = load_image(&path);
    let source_image: RawImage = convert_to_raw_image(&rgb_image);
    let parameters: Parameters = Parameters::default();
    let crop: Crop = Crop {
        x1: 0,
        y1: 0,
        x2: source_image.width as i32,
        y2: source_image.height as i32
    };
    let thumbnail: RawImage = convert_to_thumbnail(&rgb_image);
    AlbumImage {
        source_image,
        parameters,
        crop,
        thumbnail,
    }
}
    
fn load_image(path: &PathBuf) -> RgbImage {
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
    RgbImage {
        width: width as usize,
        height: height as usize,
        pixels: pixels
    }
}

fn convert_to_thumbnail(image: &RgbImage) -> RawImage {
    let resized_image: RgbImage = resize_to_thumbnail_size(&image);
    convert_to_raw_image(&resized_image)
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