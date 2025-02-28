use std::path::PathBuf;

use crate::{album::{AlbumImage, Crop, ImageView, Parameters}, repository::repository::AlbumPhoto, types::{RawImage, RgbImage, RgbPixel}};

use rayon::prelude::*;

pub struct AlbumImageLoader {
}

impl AlbumImageLoader {
    pub fn new() -> Self {
        Self { }
    }

    pub fn load(&self, album_photo: &AlbumPhoto) -> AlbumImage {
        let photo_id = album_photo.id;
        let path: PathBuf = PathBuf::from(&album_photo.file_name);
        let rgb_image: RgbImage = self.load_image(&path);
        let source_image: RawImage = convert_to_raw_image(&rgb_image);
        let parameters: Parameters = self.parse_parameters(&album_photo.parameters);
        let image_view: ImageView = ImageView {
            offset_x: 0.0,
            offset_y: 0.0,
            zoom: 0.0
        };
        let crop: Crop = Crop {
            center_x: (source_image.width as i32) / 2,
            center_y: (source_image.height as i32) / 2,
            width: source_image.width as i32,
            height: source_image.height as i32,
            angle_degrees: 0.0,
        };
        let thumbnail: RawImage = self.convert_to_thumbnail(&rgb_image);
        AlbumImage {
            photo_id,
            source_image,
            parameters,
            image_view,
            crop,
            thumbnail,
        }
    }
    
    pub fn load_image(&self, path: &PathBuf) -> RgbImage {
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

    fn convert_to_thumbnail(&self, image: &RgbImage) -> RawImage {
        let resized_image: RgbImage = self.resize_to_thumbnail_size(&image);
        convert_to_raw_image(&resized_image)
    }

    fn resize_to_thumbnail_size(&self, image: &RgbImage) -> RgbImage {
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

    fn parse_parameters(&self, parameters: &String) -> Parameters {
        serde_json::from_str(&parameters).ok().unwrap_or(Parameters::default())
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

fn pixel_value_to_u8(value: f32) -> u8 {
    if value <= 0.0 {
        0
    } else if value >= 1.0 {
        255
    } else {
        (value * 255.0) as u8
    }
}