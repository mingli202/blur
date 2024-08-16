use std::error::Error;

use image::ImageReader;

fn main() -> Result<(), Box<dyn Error>> {
    let original_filename = "assets/bg.png";
    let blurred_filename = "assets/bg-blur.png";

    let original = ImageReader::open(original_filename)?.decode()?;

    println!("{}x{}", original.width(), original.height());

    original.save(blurred_filename)?;

    Ok(())
}
