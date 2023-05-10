use image::{Rgb, ImageBuffer, GrayImage, Luma};
use rayon::prelude::*;

#[allow(dead_code)]
pub enum Channel {
    R,
    G,
    B
}

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