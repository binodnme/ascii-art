extern crate image;

use image::{GenericImageView};

pub fn get_dots(image_path: String) -> String {
    let img = image::open(image_path).unwrap();

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