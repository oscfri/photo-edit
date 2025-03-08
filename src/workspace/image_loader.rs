use std::path::PathBuf;

use crate::types::{RgbImage, RgbPixel};

#[derive(Clone, Debug)]
pub struct ImageLoadResult {
    pub photo_id: i32,
    pub image: RgbImage
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
    let image = RgbImage {
        width: width as usize,
        height: height as usize,
        pixels: pixels
    };
    ImageLoadResult { photo_id, image }
}