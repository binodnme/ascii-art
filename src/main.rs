extern crate image;

use ascii_art::alg::convert_image_to_ascii;
use ascii_art::string_to_image::string_to_image;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let mut filepath = "a.jpeg";
    if args.len() > 1 {
        filepath = &args[1];
    }

    let ascii_art_text = convert_image_to_ascii(&filepath.to_string());
    let image_buffer = string_to_image(ascii_art_text);
    image_buffer.save("test.jpg").unwrap()
}
