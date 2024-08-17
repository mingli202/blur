use std::error::Error;
use std::path::Path;

use grid::*;
use image::{DynamicImage, ImageBuffer, Rgb};

fn gaussian(x: i32, y: i32, sigma: f64) -> f64 {
    (-(x.pow(2) + y.pow(2)) as f64 / (2.0 * sigma * sigma)).exp()
        / (2.0 * std::f64::consts::PI * sigma * sigma)
}

fn get_gaussian_matrix(radius: u8, sigma: f64) -> Grid<f64> {
    if radius % 2 == 0 {
        panic!("Size must be an odd number");
    }

    let mid = (radius - 1) as i32 / 2;
    let mut matrix = grid![];

    for x in -mid..=mid {
        for y in -mid..=mid {
            let el = matrix
                .get_mut(x + mid, y + mid)
                .expect("Index out of bounds");
            *el = gaussian(x, y, sigma);
        }
    }

    matrix
}

fn blur(radius: u8, sigma: f64, mut img_buf: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) {
    let m = get_gaussian_matrix(radius, sigma);

    for (x, y, pixel) in img_buf.enumerate_pixels_mut() {
        let rgb: [u8; 3] = [0, 0, 0];
        *pixel = image::Rgb(rgb);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let original_path = Path::new("assets/bg.jpg");
    let blurred_path = Path::new("assets/blurred.jpg");

    let original_img: DynamicImage = image::open(original_path)?;

    let mut img_buf = ImageBuffer::new(original_img.width(), original_img.height());
    blur(5, 10.0, &mut img_buf);

    img_buf.save(blurred_path)?;

    Ok(())
}
