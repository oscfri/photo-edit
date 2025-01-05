use crate::types::*;

use rayon::prelude::*;

pub fn brightness(image: &mut LabImage, value: f32) {
    image.pixels.par_iter_mut()
        .for_each(|pixel| {
            pixel.lightness += value;
        });
}

pub fn contrast(image: &mut LabImage, value: f32) {
    let adjusted_value: f32 = (value + 100.0) / 100.0;
    image.pixels.par_iter_mut()
        .for_each(|pixel| {
            pixel.lightness = (pixel.lightness - 50.0) * adjusted_value + 50.0;
        });
}

pub fn tint(image: &mut LabImage, value: f32) {
    image.pixels.par_iter_mut()
        .for_each(|pixel| {
            pixel.tint += value;
        });
}

pub fn temperature(image: &mut LabImage, value: f32) {
    image.pixels.par_iter_mut()
        .for_each(|pixel| {
            pixel.temperature += value;
        });
}