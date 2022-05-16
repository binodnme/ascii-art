extern crate image;

use image::codecs::gif::{GifDecoder, GifEncoder};
use image::gif::Repeat;
use image::{AnimationDecoder, DynamicImage, Frame, ImageBuffer, Rgb};

use crate::alg::{convert_bytes_to_ascii, convert_rgb_image_to_ascii};
use crate::string_to_image::ascii_to_image;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

pub fn convert_image_to_ascii_image(bytes: &[u8]) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    log!("converting image to ascii");
    println!("converting image to ascii");
    let ascii_art_text = convert_bytes_to_ascii(bytes);
    ascii_to_image(&ascii_art_text)
}

pub fn convert_gif_to_ascii_gif(bytes: &[u8]) -> Vec<u8> {
    log!("converting gif to ascii gif");
    let decoder = GifDecoder::new(bytes).unwrap();
    let frames = decoder.into_frames();

    let mut new_frames = vec![];
    for (_i, frame) in frames.into_iter().enumerate() {
        let rgba_image = frame.unwrap().buffer().to_owned();
        let vec = convert_rgb_image_to_ascii(&DynamicImage::ImageRgba8(rgba_image).to_rgb8());
        let buffer = ascii_to_image(&vec);
        let img = DynamicImage::ImageRgb8(buffer);
        new_frames.push(Frame::new(img.to_rgba8()))
    }

    let mut bytes: Vec<u8> = Vec::new();
    {
        log!("encoding gif");
        let mut encoder = GifEncoder::new(&mut bytes);
        encoder.set_repeat(Repeat::Infinite).unwrap();
        encoder.encode_frames(new_frames).unwrap();
    }

    bytes
}
