use image::{open, DynamicImage, GenericImageView, ImageBuffer, ImageOutputFormat, Rgba, RgbaImage};
use std::env;
use std::fs::File;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: image_grayscale <input_image> <output_image>");
        return;
    }

    let input_path = &args[1];
    let output_path = &args[2];

    let img = open(input_path).expect("Failed to open input image");
    let grayscale_img = apply_grayscale(&img);

    save_image(&grayscale_img, output_path).expect("Failed to save output image");
    println!("Grayscale image saved to {}", output_path);
}

fn apply_grayscale(img: &DynamicImage) -> RgbaImage {
    let (width, height) = img.dimensions();
    let mut grayscale_img = ImageBuffer::new(width, height);

    for (x, y, pixel) in img.pixels() {
        let rgba = pixel.0;
        let avg = ((rgba[0] as u32 + rgba[1] as u32 + rgba[2] as u32) / 3) as u8;
        grayscale_img.put_pixel(x, y, Rgba([avg, avg, avg, rgba[3]]));
    }

    grayscale_img
}

fn save_image(img: &RgbaImage, path: &str) -> Result<(), image::ImageError> {
    let output_file = File::create(&Path::new(path)).expect("Failed to create output file");
    img.write_to(&mut std::io::BufWriter::new(output_file), ImageOutputFormat::Png)
}
