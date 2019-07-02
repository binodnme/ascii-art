extern crate image;

use image::{GenericImageView, Rgba};


// naive implementation of ascii art
// only dot '.' is used to draw the image
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