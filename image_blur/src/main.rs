use image::{open, DynamicImage, GenericImageView, ImageOutputFormat};
use imageproc::filter::gaussian_blur_f32;
use std::env;
use std::fs::File;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: image_blur <input_image> <output_image>");
        return;
    }

    let input_path = &args[1];
    let output_path = &args[2];

    let img = open(input_path).expect("Failed to open input image");
    let blurred_img = blur_image(&img, 10.0);

    save_image(&blurred_img, output_path).expect("Failed to save output image");
    println!("Blurred image saved to {}", output_path);
}

fn blur_image(img: &DynamicImage, sigma: f32) -> DynamicImage {
    let img = img.to_rgb8();
    let blurred = gaussian_blur_f32(&img, sigma);
    DynamicImage::ImageRgb8(blurred)
}

fn save_image(img: &DynamicImage, path: &str) -> Result<(), image::ImageError> {
    let output_file = File::create(&Path::new(path)).expect("Failed to create output file");
    img.write_to(&mut std::io::BufWriter::new(output_file), ImageOutputFormat::Png)
}
