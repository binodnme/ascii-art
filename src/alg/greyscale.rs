//   ..  ..  ..  ..  ..  ..  ..  ..  ..  ..  ..  ..  ..  ..  ..  ..
//   ..  ..  ..  ..  .@  .@  .@  .@  @.  @.  @.  @.  @@  @@  @@  @@
//   ..  .@  @.  @@  ..  .@  @.  @@  ..  .@  @.  @@  ..  .@  @.  @@
//       ,   .   _   -   i   v   g   -   c   i   s   =   e   z   m

//   .@  .@  .@  .@  .@  .@  .@  .@  .@  .@  .@  .@  .@  .@  .@  .@
//   ..  ..  ..  ..  .@  .@  .@  .@  @.  @.  @.  @.  @@  @@  @@  @@
//   ..  .@  @.  @@  ..  .@  @.  @@  ..  .@  @.  @@  ..  .@  @.  @@
//   '   !   /   2   !   ]   /   d   /   (   /   K   Y   4   Z   W

//   @.  @.  @.  @.  @.  @.  @.  @.  @.  @.  @.  @.  @.  @.  @.  @.
//   ..  ..  ..  ..  .@  .@  .@  .@  @.  @.  @.  @.  @@  @@  @@  @@
//   ..  .@  @.  @@  ..  .@  @.  @@  ..  .@  @.  @@  ..  .@  @.  @@
//   `   \   |   L   \   \   )   G   !   t   [   b   +   N   D   W

//   @@  @@  @@  @@  @@  @@  @@  @@  @@  @@  @@  @@  @@  @@  @@  @@
//   ..  ..  ..  ..  .@  .@  .@  .@  @.  @.  @.  @.  @@  @@  @@  @@
//   ..  .@  @.  @@  ..  .@  @.  @@  ..  .@  @.  @@  ..  .@  @.  @@
//   ~   T   7   X   V   Y   Z   8   f   5   P   K   *   M   A   @

extern crate image;

use image::{GenericImageView, Rgba};
use image::imageops::{FilterType};



// grey scale threshold value for black white algorithm
const GREY_SCALE_THRESHOLD: u8 = 200;


//uses black and white algorithm
pub fn get_output(image_path: String) -> String {
    let mut img = image::open(image_path).unwrap();

    img = img.resize(250, 250, FilterType::Triangle);
    let mut output = String::new();

    let (width, height) = img.dimensions();

    let mut pos_x = 0;
    let mut pos_y = 0;

    while pos_y < height - 5 {
        if pos_x >= width - 5 {
            pos_x = 0;
            pos_y+=3;
            output += "\n"
        }

        //take 4 adjacent pixels at a time
        let _0 = img.get_pixel(pos_x, pos_y);
        let _1 = img.get_pixel(pos_x +1, pos_y);
        let _2 = img.get_pixel(pos_x + 1, pos_y +1);
        let _3 = img.get_pixel(pos_x + 1, pos_y +2);
        let _4 = img.get_pixel(pos_x, pos_y +2);
        let _5 = img.get_pixel(pos_x, pos_y +1);
        
        output += &get_character(_0, _1, _2, _3, _4, _5);

        pos_x += 2;

    }

    return output;
}

// returns a ascii charater representing four corresponding pixels
fn get_character(_0: Rgba<u8>, _1: Rgba<u8>, _2: Rgba<u8>, _3: Rgba<u8>, _4: Rgba<u8>, _5: Rgba<u8>) -> String {

    let quad = (
        is_below_threshold(_0),
        is_below_threshold(_1),
        is_below_threshold(_2),
        is_below_threshold(_3),
        is_below_threshold(_4),
        is_below_threshold(_5),
        );

    let value = match quad {
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

        _ => " "
    };

    return String::from(value);
}

fn is_below_threshold(color: Rgba<u8>) -> u8 {
    // do not consider pixel if it is transparent
    if color.data[3] == 0 {
        return 1
    }

    if color.data[0] < GREY_SCALE_THRESHOLD
        && color.data[1] < GREY_SCALE_THRESHOLD
        && color.data[2] < GREY_SCALE_THRESHOLD {
        0
    } else {
        1
    }
}