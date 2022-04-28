extern crate image;

/// Algorithm module
pub mod alg;
pub mod string_to_image;
pub mod utilities;
// pub mod main;

use crate::alg::convert_image_to_ascii;
use crate::utilities::{convert_gif_to_ascii_gif, convert_image_to_ascii_image};
use image::DynamicImage::ImageRgba8;
use image::{DynamicImage, EncodableLayout, ImageFormat};
use wasm_bindgen::prelude::*;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
pub fn create_ascii_art_from_png(bytes: &[u8]) -> Vec<u8> {
    log!("creating ascii art");

    let ascii_image = convert_image_to_ascii_image(bytes);
    let dynamic_image = DynamicImage::ImageRgb8(ascii_image);
    let mut bytes: Vec<u8> = Vec::new();
    dynamic_image
        .write_to(&mut bytes, image::ImageOutputFormat::from(ImageFormat::Png))
        .expect("Error while writing");
    bytes
}

#[wasm_bindgen]
pub fn create_ascii_art_from_gif(bytes: &[u8]) -> Vec<u8> {
    log!("creating ascii art");

    convert_gif_to_ascii_gif(bytes)
}
