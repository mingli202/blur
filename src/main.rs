use blur::*;

use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let Opts {
        radius,
        sigma,
        original: original_path,
        blurred: blurred_path,
        n_threads,
    } = Opts::new(env::args())?;

    let original_img = image::open(original_path)?.to_rgb8();

    let img_buf = blur_async(radius, sigma, n_threads, original_img);

    img_buf.save(blurred_path)?;

    Ok(())
}
