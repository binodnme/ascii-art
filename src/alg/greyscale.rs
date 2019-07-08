extern crate image;

use image::imageops::FilterType;
use image::{GenericImageView, Rgba};

/// grey scale threshold value algorithm
const GREY_SCALE_THRESHOLD: u8 = 200;

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

/// returns the ascii-art as String
pub fn get_output(image_path: String) -> String {
    let mut img = image::open(image_path).unwrap();

    // img = img.resize(100, 100, FilterType::Triangle);
    let mut output = String::new();

    let (width, height) = img.dimensions();

    let mut pos_x = 0;
    let mut pos_y = 0;

    while pos_y < height - 5 {
        if pos_x >= width - 5 {
            pos_x = 0;
            pos_y += 3;
            output += "\n"
        }

        //take 6 adjacent pixels at a time
        let x_y = img.get_pixel(pos_x, pos_y);
        let x_1_y = img.get_pixel(pos_x + 1, pos_y);
        let x_1_y_1 = img.get_pixel(pos_x + 1, pos_y + 1);
        let x_1_y_2 = img.get_pixel(pos_x + 1, pos_y + 2);
        let x_y_2 = img.get_pixel(pos_x, pos_y + 2);
        let x_y_1 = img.get_pixel(pos_x, pos_y + 1);

        let pb = PixelBlock {
            x_y: x_y,
            x_1_y: x_1_y,
            x_1_y_1: x_1_y_1,
            x_1_y_2: x_1_y_2,
            x_y_2: x_y_2,
            x_y_1: x_y_1,
        };

        output += &get_character(pb, INVERT_COLOR);

        pos_x += 2;
    }

    return output;
}

//returns a ascii charater representing four corresponding pixels
fn get_character(pb: PixelBlock, invert_color: bool) -> String {
    let mut block = BlockThresholdFlag {
        x_y: is_below_threshold(pb.x_y),
        x_1_y: is_below_threshold(pb.x_1_y),
        x_1_y_1: is_below_threshold(pb.x_1_y_1),
        x_1_y_2: is_below_threshold(pb.x_1_y_2),
        x_y_2: is_below_threshold(pb.x_y_2),
        x_y_1: is_below_threshold(pb.x_y_1),
    };

    if invert_color {
        block = invert_block_threshold_value(block);
    }

    let tuple_value = (block.x_y, block.x_1_y, block.x_1_y_1, block.x_1_y_2, block.x_y_2, block.x_y_1);

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
    let (r, g, b, alpha) = (color.data[0], color.data[1], color.data[2], color.data[3]);

    let output;
    if alpha == 0 {
        output = 0;
    }else if r < GREY_SCALE_THRESHOLD && g < GREY_SCALE_THRESHOLD && b < GREY_SCALE_THRESHOLD {
        output = 1;
    } else {
        output = 0;
    }

    output
}

fn invert_block_threshold_value(mut block: BlockThresholdFlag) -> BlockThresholdFlag {
    block.x_y = if block.x_y == 0 {1} else {0};
    block.x_1_y = if block.x_1_y == 0 {1} else {0};
    block.x_1_y_1 = if block.x_1_y_1 == 0 {1} else {0};
    block.x_1_y_2 = if block.x_1_y_2 == 0 {1} else {0};
    block.x_y_1 = if block.x_y_1 == 0 {1} else {0};
    block.x_y_2 = if block.x_y_2 == 0 {1} else {0};
    block
}

#[cfg(test)]
#[test]
fn should_be_below_threshold() {
    let color: Rgba<u8> = Rgba {
        data: [20, 20, 20, 255],
    };

    assert_eq!(is_below_threshold(color), 0)
}
