extern crate image;

/// Algorithm module
pub mod alg;
pub mod string_to_image;
pub mod utilities;
// pub mod main;

use crate::alg::convert_image_to_ascii;
use crate::utilities::{convert_image_to_ascii_image, convert_image_to_ascii_image_1};
use image::{DynamicImage, EncodableLayout, ImageFormat};
use image::DynamicImage::ImageRgba8;
use wasm_bindgen::prelude::*;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
extern "C" {
    type Buffer;
}

#[wasm_bindgen(module = "fs")]
extern "C" {
    #[wasm_bindgen(js_name = readFileSync, catch)]
    fn read_file(path: &str) -> Result<Buffer, JsValue>;
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, from ascii-art!");
}

#[wasm_bindgen]
pub fn create_ascii_art_(file_path: String) -> String {
    log!("{}", &file_path);
    file_path
    // convert_image_to_ascii_image(&file_path)
}

#[wasm_bindgen]
pub fn create_ascii_art(bytes: &[u8]) -> Vec<u8> {
    log!("creating ascii art");

    let ascii_image = convert_image_to_ascii_image_1(bytes);
    let dynamic_image = DynamicImage::ImageRgb8(ascii_image);
    let mut bytes: Vec<u8> = Vec::new();
    dynamic_image.write_to(&mut bytes, image::ImageOutputFormat::from(ImageFormat::Png)).expect("Error while writing");
    bytes
}
