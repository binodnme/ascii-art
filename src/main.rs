extern crate image;

use std::env;
use std::fs::File;

use image::{AnimationDecoder, DynamicImage, Frame};
use image::codecs::gif::{GifDecoder, GifEncoder};
use image::gif::Repeat;

use ascii_art::alg::{convert_image_to_ascii, convert_rgb_image_to_ascii};
use ascii_art::string_to_image::ascii_to_image;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut filepath = "a.jpeg";
    if args.len() > 1 {
        filepath = &args[1];
    }

    convert_image_to_ascii_image(&filepath.to_string());
    convert_gif_to_ascii_gif(&filepath.to_string());
}

fn convert_image_to_ascii_image(path: &String) {
    let ascii_art_text = convert_image_to_ascii(path);
    let image_buffer = ascii_to_image(&ascii_art_text);
    image_buffer.save("test.jpg").unwrap();
}

fn convert_gif_to_ascii_gif(path: &String) {
    let file = File::open(path).unwrap();
    let decoder = GifDecoder::new(file).unwrap();
    let frames = decoder.into_frames();
    let new_frames = frames.into_iter().map(|frame| {
        let x = frame.unwrap();
        let b = x.buffer().clone();
        let vec = convert_rgb_image_to_ascii(&DynamicImage::ImageRgba8(b).to_rgb8());
        let buffer = ascii_to_image(&vec);
        let img = DynamicImage::ImageRgb8(buffer);
        Frame::new(img.to_rgba8())
    }).collect::<Vec<Frame>>();

    let file_out = File::options().read(true).write(true).open("output.gif").unwrap();
    let mut encoder = GifEncoder::new(file_out);
    encoder.set_repeat(Repeat::Infinite).unwrap();
    encoder.encode_frames(new_frames).unwrap();
}
