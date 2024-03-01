use base64::Engine;
use image::GenericImage;
use std::io::Cursor;

pub fn raw_bytes_to_base64_png(bytes: Vec<u8>, width: usize, height: usize) -> Vec<u8> {
    let  bytes = raw_bytes_to_png(bytes, width, height);

    let mut output: Vec<u8> = Default::default();
    output.resize(bytes.len() * 4 / 3 + 4, 0);
    let bytes_written = base64::prelude::BASE64_STANDARD.encode_slice(&bytes, output.as_mut()).unwrap();
    output.truncate(bytes_written);
    output
}


pub fn raw_bytes_to_png(bytes: Vec<u8>, width: usize, height: usize) -> Vec<u8> {
    assert_eq!(bytes.len(), width * height * 4);

    let mut image = image::DynamicImage::new_rgba8(width as u32, height as u32);
    for y in 0..height {
        for x in 0..width {
            let i = (y * width + x) * 4;
            image.put_pixel(
                x as u32,
                y as u32,
                image::Rgba::<u8>([bytes[i], bytes[i + 1], bytes[i + 2], bytes[i + 3]]),
            );
        }
    }

    let mut bytes: Vec<u8> = Vec::new();
    image
        .write_to(&mut Cursor::new(&mut bytes), image::ImageFormat::Png)
        .unwrap();
    bytes
}
