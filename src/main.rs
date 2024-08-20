use blur::*;

use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let (original_path, blurred_path) = get_fnames(env::args());

    let original_img = image::open(original_path)?.to_rgb8();

    let sigma = 10.0;
    let radius = 2;

    let img_buf = blur_async(radius, sigma, original_img);

    img_buf.save(blurred_path)?;

    Ok(())
}
