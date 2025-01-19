use std::path::PathBuf;

use crate::types::*;
use crate::conversions;
use crate::pixelwise;

use rayon::prelude::*;

pub struct Album {
    pub images: Vec<AlbumImage>
}

pub struct AlbumImage {
    pub source_image: LabImage,
    pub parameters: Parameters,
    pub thumbnail: RawImage
}

impl AlbumImage {
    pub fn into_work_image(&self) -> WorkImage {
        WorkImage {
            source_image: self.source_image.clone(),
            parameters: self.parameters.clone()
        }
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> Option<LabPixel> {
        if x < self.source_image.width && y < self.source_image.height {
            Some(self.source_image.pixels[y * self.source_image.width + x].clone())
        } else {
            None
        }
    }
}

// TODO: Might not be a good idea to store all full images as is. Should probably only refer to a path
#[derive(Clone)]
pub struct WorkImage {
    pub source_image: LabImage,
    pub parameters: Parameters
}

#[derive(Debug, Default, Clone)]
pub struct Parameters {
    pub brightness: f32,
    pub contrast: f32,
    pub tint: f32,
    pub temperature: f32,
    pub saturation: f32
}

pub fn load_album(file_paths: &Vec<PathBuf>) -> Album {
    let images: Vec<AlbumImage> = file_paths.iter().map(load_album_image).collect();
    Album {
        images: images
    }
}

impl WorkImage {
    pub fn apply_parameters(self) -> RawImage {
        let mut image: LabImage = self.source_image;
    
        // NOTE: This takes ~30ms
        pixelwise::contrast(&mut image, self.parameters.contrast);
        pixelwise::brightness(&mut image, self.parameters.brightness);
        pixelwise::saturation(&mut image, self.parameters.saturation);
        pixelwise::tint(&mut image, self.parameters.tint);
        pixelwise::temperature(&mut image, self.parameters.temperature);
        // NOTE: This takes ~70ms
        let rgb_image: RgbImage = conversions::lab_image_to_rgb(&image);
    
        // NOTE: This takes ~80ms
        convert_to_raw_image(&rgb_image)
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
    let image: LabImage = load_image_as_lab(&path);
    let parameters: Parameters = Parameters::default();
    let thumbnail: RawImage = convert_to_thumbnail(&image);
    AlbumImage {
        source_image: image,
        parameters: parameters,
        thumbnail
    }
}

fn load_image_as_lab(path: &PathBuf) -> LabImage {
    let image: RgbImage = load_image(path);
    conversions::rgb_image_to_lab(&image)
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

fn convert_to_thumbnail(image: &LabImage) -> RawImage {
    let resized_image: LabImage = resize_to_thumbnail_size(&image);
    let rgb_image: RgbImage = conversions::lab_image_to_rgb(&resized_image);
    convert_to_raw_image(&rgb_image)
}

fn resize_to_thumbnail_size(image: &LabImage) -> LabImage {
    let target_size: usize = 100;
    let width_skip: usize = std::cmp::max(1, image.width / target_size);
    let height_skip: usize = std::cmp::max(1, image.height / target_size);

    let target_width: usize = image.width / width_skip;
    let target_height: usize = image.height / height_skip;
    let mut pixels: Vec<LabPixel> = Vec::new();

    for h in 0..target_height {
        for w in 0..target_width {
            pixels.push(image.pixels[(h * height_skip) * image.width + w * width_skip].clone());
        }
    }

    LabImage {
        width: target_width,
        height: target_height,
        pixels: pixels
    }
}