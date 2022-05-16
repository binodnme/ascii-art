extern crate image;


use std::env;

use image::{GenericImageView, ImageFormat};

// use crate::utils1::

fn main() {

    // let bytes: Vec<u8> = std::fs::read("img_2.png").unwrap();

    let args: Vec<String> = env::args().collect();

    let mut filepath = "a.jpeg";
    if args.len() > 1 {
        filepath = &args[1];
    }

    println!("{}", filepath);
    let img = image::open(&filepath.to_string()).unwrap();
    println!("{:?}", img.dimensions());
    let img_bytes = img.into_bytes();

    let img = image::load_from_memory_with_format(&img_bytes, ImageFormat::Png).unwrap() ;
    println!("{:?}", img.dimensions())


    // let mut reader = Reader::new(Cursor::new(&img_bytes))
    //     .with_guessed_format()
    //     .expect("Cursor io never fails");
    //
    // assert_eq!(reader.format(), Some(ImageFormat::Png));
    //
    // let image = reader.decode().unwrap();
    // println!("{:?}", image.dimensions())


    // let output = convert_image_to_ascii_image_1(img.as_bytes());

    // println!("{}", output);

    // convert_image_to_ascii_image(&"/home/kaala/projects/ascii-art/img_2.png".to_string())
    // (&filepath.to_string());
    // convert_gif_to_ascii_gif(&filepath.to_string());
    // convert_image_to_ascii_image(&filepath.to_string())
}
