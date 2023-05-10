use image::{Rgb, ImageBuffer, GrayImage, Luma};
use rayon::prelude::*;

#[allow(dead_code)]
pub enum Channel {
    R,
    G,
    B
}

#[allow(dead_code)]
pub fn get_channel_image(rgb: ImageBuffer<Rgb<u8>, Vec<u8>>, channel: Channel)
    -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let width = rgb.width();
    let height = rgb.height();

    let color = match channel {
        Channel::R => 0,
        Channel::G => 1,
        Channel::B => 2,
    };

    let mut img = GrayImage::new(width, height);
    img.enumerate_pixels_mut()
        .collect::<Vec<(u32, u32, &mut Luma<u8>)>>()
        .par_iter_mut()
        .for_each(|(x, y, pixel)| {
            let val = rgb.get_pixel(*x, *y);
            pixel[0] = val[color];
        });
    img
}

#[allow(dead_code)]
pub fn gray_scale(rgb: ImageBuffer<Rgb<u8>, Vec<u8>>)
    -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let width = rgb.width();
    let height = rgb.height();

    let mut img = GrayImage::new(width, height);
    img.enumerate_pixels_mut()
    .collect::<Vec<(u32, u32, &mut Luma<u8>)>>()
    .par_iter_mut()
    .for_each(|(x, y, pixel)| {
        let val = rgb.get_pixel(*x, *y);
        pixel[0] = (0.2126 * val[0] as f32 + 0.7152 * val[1] as f32 + 0.0722 * val[2] as f32) as u8;
    });
    img
}

#[allow(dead_code)]
pub fn posterization(gray: ImageBuffer<Luma<u8>, Vec<u8>>, quantization_number: u8)
    -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let width = gray.width();
    let height = gray.height();

    let color = 255 / quantization_number;

    let mut img = GrayImage::new(width, height);
    img.enumerate_pixels_mut()
    .collect::<Vec<(u32, u32, &mut Luma<u8>)>>()
    .par_iter_mut()
    .for_each(|(x, y, pixel)| {
        let val = gray.get_pixel(*x, *y);
        pixel[0] = ((val[0] / color) * color + (color / 2)) as u8;
    });
    img
}
