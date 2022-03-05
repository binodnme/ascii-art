use image::{ImageBuffer, Rgba, RgbaImage};
use imageproc::drawing::draw_text_mut;
use rusttype::{Font, Scale};

pub fn string_to_image(text: Vec<String>) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let font = Vec::from(include_bytes!("UbuntuMono-R.ttf") as &[u8]);
    let font = Font::try_from_vec(font).unwrap();

    let char_height = 6.0;
    let scale = Scale {
        x: char_height * 3.0,
        y: char_height * 2.0,
    };

    let buffer_width = (char_height as u32) * 3 * (text.get(0).unwrap().len() as u32) / 2;
    let buffer_height = (char_height as u32) * 2 * (text.len() as u32);

    println!("{} {}", text.get(0).unwrap().len(), text.len());
    println!("{} {}", buffer_width, buffer_height);
    let mut image_buffer = RgbaImage::new(buffer_width, buffer_height);

    let color: Rgba<u8> = Rgba {
        0: [255, 255, 255, 255],
    };

    for (i, line) in text.iter().enumerate() {
        draw_text_mut(&mut image_buffer, color, 0, (i * 12) as u32, scale, &font, line);
    }

    image_buffer
}