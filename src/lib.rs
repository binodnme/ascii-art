extern crate image;

use image::{GenericImageView, Rgba};

// greyscale threshold value for black white algorithm
const BLACK_WHITE_THRESHOLD: u8 = 50;

pub fn get_dots(image_path: String) -> String {
    let mut img = image::open(image_path).unwrap();
    img = img.thumbnail_exact(100, 80);

    let mut output = String::new();
    for (x, _, rgba) in img.pixels() {
        if x == img.dimensions().0 -1 {
            output += "\n";
        }

        let limit = 100;
        if rgba.data[0] < limit && rgba.data[0] < limit && rgba.data[0] < limit {
            output += ".";
        } else {
            output += " ";
        }
    }

    return output;
}

//uses black and white algorithm
pub fn black_white(image_path: String) -> String {
    let img = image::open(image_path).unwrap();

    let mut output = String::new();

    let (width, height) = img.dimensions();

    let mut pos_x = 0;
    let mut pos_y = 0;

    while pos_y < height - 3 {
        if pos_x >= width - 3 {
            pos_x = 0;
            pos_y+=2;
            output += "\n"
        }

        //take 4 adjacent pixels at a time
        let _0 = img.get_pixel(pos_x, pos_y);
        let _1 = img.get_pixel(pos_x +1, pos_y);
        let _2 = img.get_pixel(pos_x, pos_y +1);
        let _3 = img.get_pixel(pos_x +1, pos_y + 1);

        output += &get_character(_0, _1, _2, _3);

        pos_x += 2;

    }

    return output;
}

// returns a ascii charater representing four corresponding pixels
fn get_character(_0: Rgba<u8>, _1: Rgba<u8>, _2: Rgba<u8>, _3: Rgba<u8>) -> String {

    let quad = (
        is_below_threshold(_0),
        is_below_threshold(_1),
        is_below_threshold(_2),
        is_below_threshold(_3)
        );

    let value = match quad {
        (0, 0, 0, 0) => " ",

        (1, 0, 0, 0) => "`",
        (0, 1, 0, 0) => "'",
        (0, 0, 1, 0) => ",",
        (0, 0, 0, 1) => ".",

        (1, 1, 0, 0) => "\"",
        (1, 0, 1, 0) => "\\",
        (1, 0, 0, 1) => "(",
        (0, 1, 1, 0) => ")",
        (0, 1, 0, 1) => "/",
        (0, 0, 1, 1) => "_",

        (1, 1, 1, 0) => "7",
        (1, 1, 0, 1) => "P",
        (1, 0, 1, 1) => "L",
        (0, 1, 1, 1) => "J",

        (1, 1, 1, 1) => "8",
        _ => " "
    };

    return String::from(value);
}

fn is_below_threshold(color: Rgba<u8>) -> u8 {
    let limit = BLACK_WHITE_THRESHOLD;
    
    // do not consider pixel if it is transparent
    if color.data[3] == 0 {
        return 0
    }

    if color.data[0] < limit && color.data[1] < limit && color.data[2] < limit {
        1
    } else {
        0
    }
}