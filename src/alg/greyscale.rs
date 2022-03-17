extern crate image;

use crate::Buffer;
use image::error::UnsupportedErrorKind::Format;
use image::imageops::FilterType;
use image::{buffer, DynamicImage, GenericImageView, ImageBuffer, ImageFormat, RgbImage, Rgba};
use web_sys::console::log;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}
/// grey scale threshold value algorithm
const GREY_SCALE_THRESHOLD: u8 = 75;

/// color invert option
const INVERT_COLOR: bool = true;

///Block contains the 6 adjacent pixels
/// (x,y), (x + 1, y)
/// (x, y + 1), (x +1 , y + 1)
/// (x, y + 2), (x + 1, y + 2)
struct PixelBlock {
    x_y: Rgba<u8>,
    x_1_y: Rgba<u8>,
    x_1_y_1: Rgba<u8>,
    x_1_y_2: Rgba<u8>,
    x_y_2: Rgba<u8>,
    x_y_1: Rgba<u8>,
}

///Block contains the threshold flag of 6 adjacent pixels
/// (x,y), (x + 1, y)
/// (x, y + 1), (x +1 , y + 1)
/// (x, y + 2), (x + 1, y + 2)
struct BlockThresholdFlag {
    x_y: u8,
    x_1_y: u8,
    x_1_y_1: u8,
    x_1_y_2: u8,
    x_y_2: u8,
    x_y_1: u8,
}

pub fn convert_image_to_ascii(image_path: &str) -> Vec<String> {
    log!("converting rgb image to ascii 1");
    log!("{}", &image_path);
    let img = image::open("/home/kaala/projects/ascii-art/img_2.png").unwrap();
    log!("image successfully opened..");
    convert_rgb_image_to_ascii(&img.to_rgb8())
}

pub fn convert_image_buffer_to_ascii(bytes: &[u8]) -> Vec<String> {
    println!("converting rgb image to ascii 1");
    log!("converting rgb image to ascii 1");
    let img = match image::load_from_memory(bytes) {
        Ok(img) => img,
        Err(error) => {
            panic!("{:?}", error)
        }
    };

    // let img = image::open("/home/kaala/projects/ascii-art/img_2.png").unwrap();
    log!("image successfully opened..");
    convert_rgb_image_to_ascii(&img.to_rgb8())
}

/// returns the ascii-art as String
pub fn convert_rgb_image_to_ascii(img: &RgbImage) -> Vec<String> {
    // log!("converting rgb image to ascii 2");
    let buffer = img.clone();
    let mut img = DynamicImage::ImageRgb8(buffer);

    img = img.resize(400, 250, FilterType::Triangle);

    let (width, height) = img.dimensions();
    let mut result: Vec<String> = vec![];
    let mut pos_x = 0;
    let mut pos_y = 0;
    // TODO: remove magic number
    let mut output = String::new();
    while pos_y < height - 5 {
        if pos_x >= width - 5 {
            pos_x = 0;
            pos_y += 3;
            // output += "\n"
            result.push(output);
            output = "".to_string();
        }

        //take 6 adjacent pixels at a time
        let x_y = img.get_pixel(pos_x, pos_y);
        let x_1_y = img.get_pixel(pos_x + 1, pos_y);
        let x_1_y_1 = img.get_pixel(pos_x + 1, pos_y + 1);
        let x_1_y_2 = img.get_pixel(pos_x + 1, pos_y + 2);
        let x_y_2 = img.get_pixel(pos_x, pos_y + 2);
        let x_y_1 = img.get_pixel(pos_x, pos_y + 1);

        let pb = PixelBlock {
            x_y,
            x_1_y,
            x_1_y_1,
            x_1_y_2,
            x_y_2,
            x_y_1,
        };

        // result.append(&get_character(&pb, INVERT_COLOR));
        output.push_str(&get_character(&pb, INVERT_COLOR));

        pos_x += 2;
    }

    return result;
}

//returns a ascii charater representing four corresponding pixels
fn get_character(pb: &PixelBlock, invert_color: bool) -> String {
    let mut block = BlockThresholdFlag {
        x_y: is_below_threshold(pb.x_y),
        x_1_y: is_below_threshold(pb.x_1_y),
        x_1_y_1: is_below_threshold(pb.x_1_y_1),
        x_1_y_2: is_below_threshold(pb.x_1_y_2),
        x_y_2: is_below_threshold(pb.x_y_2),
        x_y_1: is_below_threshold(pb.x_y_1),
    };

    if invert_color {
        invert_block_threshold_value(&mut block);
    }

    let tuple_value = (
        block.x_y,
        block.x_1_y,
        block.x_1_y_1,
        block.x_1_y_2,
        block.x_y_2,
        block.x_y_1,
    );

    let value = match tuple_value {
        (0, 0, 0, 0, 0, 0) => " ",
        (0, 0, 0, 1, 0, 0) => ",",
        (0, 0, 0, 0, 1, 0) => ".",
        (0, 0, 0, 1, 1, 0) => "_",

        (0, 0, 1, 0, 0, 0) => "-",
        (0, 0, 1, 1, 0, 0) => "i",
        (0, 0, 1, 0, 1, 0) => "v",
        (0, 0, 1, 1, 1, 0) => "g",

        (0, 0, 0, 0, 0, 1) => "-",
        (0, 0, 0, 1, 0, 1) => "c",
        (0, 0, 0, 0, 1, 1) => "i",
        (0, 0, 0, 1, 1, 1) => "s",

        (0, 0, 1, 0, 0, 1) => "=",
        (0, 0, 1, 1, 0, 1) => "e",
        (0, 0, 1, 0, 1, 1) => "z",
        (0, 0, 1, 1, 1, 1) => "m",

        (0, 1, 0, 0, 0, 0) => "'",
        (0, 1, 0, 1, 0, 0) => "!",
        (0, 1, 0, 0, 1, 0) => "/",
        (0, 1, 0, 1, 1, 0) => "2",

        (0, 1, 1, 0, 0, 0) => "!",
        (0, 1, 1, 1, 0, 0) => "]",
        (0, 1, 1, 0, 1, 0) => "/",
        (0, 1, 1, 1, 1, 0) => "d",

        (0, 1, 0, 0, 0, 1) => "/",
        (0, 1, 0, 1, 0, 1) => "(",
        (0, 1, 0, 0, 1, 1) => "/",
        (0, 1, 0, 1, 1, 1) => "K",

        (0, 1, 1, 0, 0, 1) => "Y",
        (0, 1, 1, 1, 0, 1) => "4",
        (0, 1, 1, 0, 1, 1) => "Z",
        (0, 1, 1, 1, 1, 1) => "W",

        (1, 0, 0, 0, 0, 0) => "`",
        (1, 0, 0, 1, 0, 0) => "\\",
        (1, 0, 0, 0, 1, 0) => "|",
        (1, 0, 0, 1, 1, 0) => "L",

        (1, 0, 1, 0, 0, 0) => "\\",
        (1, 0, 1, 1, 0, 0) => "\\",
        (1, 0, 1, 0, 1, 0) => ")",
        (1, 0, 1, 1, 1, 0) => "G",

        (1, 0, 0, 0, 0, 1) => "!",
        (1, 0, 0, 1, 0, 1) => "t",
        (1, 0, 0, 0, 1, 1) => "[",
        (1, 0, 0, 1, 1, 1) => "b",

        (1, 0, 1, 0, 0, 1) => "+",
        (1, 0, 1, 1, 0, 1) => "N",
        (1, 0, 1, 0, 1, 1) => "D",
        (1, 0, 1, 1, 1, 1) => "W",

        (1, 1, 0, 0, 0, 0) => "~",
        (1, 1, 0, 1, 0, 0) => "T",
        (1, 1, 0, 0, 1, 0) => "7",
        (1, 1, 0, 1, 1, 0) => "X",

        (1, 1, 1, 0, 0, 0) => "V",
        (1, 1, 1, 1, 0, 0) => "Y",
        (1, 1, 1, 0, 1, 0) => "Z",
        (1, 1, 1, 1, 1, 0) => "8",

        (1, 1, 0, 0, 0, 1) => "f",
        (1, 1, 0, 1, 0, 1) => "5",
        (1, 1, 0, 0, 1, 1) => "P",
        (1, 1, 0, 1, 1, 1) => "K",

        (1, 1, 1, 0, 0, 1) => "*",
        (1, 1, 1, 1, 0, 1) => "M",
        (1, 1, 1, 0, 1, 1) => "A",
        (1, 1, 1, 1, 1, 1) => "@",

        _ => " ",
    };

    return String::from(value);
}

fn is_below_threshold(color: Rgba<u8>) -> u8 {
    let (r, g, b, alpha) = (color.0[0], color.0[1], color.0[2], color.0[3]);

    let output;
    if alpha == 0 {
        output = 0;
    } else if r < GREY_SCALE_THRESHOLD && g < GREY_SCALE_THRESHOLD && b < GREY_SCALE_THRESHOLD {
        output = 1;
    } else {
        output = 0;
    }

    output
}

fn invert_block_threshold_value(block: &mut BlockThresholdFlag) {
    block.x_y = if block.x_y == 0 { 1 } else { 0 };
    block.x_1_y = if block.x_1_y == 0 { 1 } else { 0 };
    block.x_1_y_1 = if block.x_1_y_1 == 0 { 1 } else { 0 };
    block.x_1_y_2 = if block.x_1_y_2 == 0 { 1 } else { 0 };
    block.x_y_1 = if block.x_y_1 == 0 { 1 } else { 0 };
    block.x_y_2 = if block.x_y_2 == 0 { 1 } else { 0 };
}

#[cfg(test)]
#[test]
fn should_be_below_threshold() {
    let color: Rgba<u8> = Rgba {
        0: [20, 20, 20, 255],
    };

    assert_eq!(is_below_threshold(color), 1)
}
