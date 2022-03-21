extern crate image;

use std::fs::File;
use std::sync::{Arc, Mutex};
use std::{env, thread};

use image::codecs::gif::{GifDecoder, GifEncoder};
use image::gif::Repeat;
use image::{AnimationDecoder, DynamicImage, Frame, ImageBuffer, Rgb};

use crate::alg::{convert_bytes_to_ascii, convert_image_to_ascii};
use crate::string_to_image::ascii_to_image;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

pub fn convert_image_to_ascii_image(bytes: &[u8]) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    log!("converting image to ascii");
    println!("converting image to ascii");
    let ascii_art_text = convert_bytes_to_ascii(bytes);
    ascii_to_image(&ascii_art_text)
}

pub fn convert_image_to_ascii_image_1(path: &String) {
    println!("converting image to ascii");
    let ascii_art_text = convert_image_to_ascii(path);
    println!("converting ascii to image");
    let image_buffer = ascii_to_image(&ascii_art_text);
    println!("{:?}", &image_buffer);
    // image_buffer.to_vec()
    image_buffer.save("test.jpg").unwrap();
}

fn convert_gif_to_ascii_gif(path: &String) {
    // let file = File::open(path).unwrap();
    // let decoder = GifDecoder::new(file).unwrap();
    // let frames = decoder.into_frames();
    //
    // let file_out = File::options()
    //     .read(true)
    //     .write(true)
    //     .open("output.gif")
    //     .unwrap();
    // let mut encoder = GifEncoder::new(file_out);
    // encoder.set_repeat(Repeat::Infinite).unwrap();
    //
    // let counter = Arc::new(Mutex::new(vec![]));
    // let mut handles = vec![];
    //
    // for (i, frame) in frames.into_iter().enumerate() {
    //     let counter = Arc::clone(&counter);
    //
    //     let handle = thread::spawn(move || {
    //         let rgba_image = frame.unwrap().buffer().to_owned();
    //
    //         let vec = convert_rgb_image_to_ascii(&DynamicImage::ImageRgba8(rgba_image).to_rgb8());
    //         let buffer = ascii_to_image(&vec);
    //
    //         let img = DynamicImage::ImageRgb8(buffer);
    //         let mut num = counter.lock().unwrap();
    //         num.push((i, Frame::new(img.to_rgba8())))
    //     });
    //
    //     handles.push(handle);
    // }
    //
    // for handle in handles {
    //     handle.join().unwrap()
    // }
    //
    // let mutex_guard = counter.lock().unwrap();
    //
    // let mut frame_vec = mutex_guard.clone();
    // frame_vec.sort_by(|a, b| a.0.cmp(&b.0));
    // let ordered_frame = frame_vec.into_iter().map(|i| i.1);
    //
    // encoder.encode_frames(ordered_frame).unwrap();
}
