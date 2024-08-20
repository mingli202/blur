use grid::*;
use image::{ImageBuffer, Rgb, RgbImage};
use std::env;
use std::sync::{mpsc, Arc};

mod threadpool;
use std::path::PathBuf;
use threadpool::*;

pub struct Opts {
    pub radius: u8,
    pub sigma: f64,
    pub original: PathBuf,
    pub blurred: PathBuf,
}

impl Opts {
    pub fn new(mut cli_opts: env::Args) -> Opts {
        let mut radius = 10;
        let mut sigma = 10.0;
        let mut original: Option<PathBuf> = None;
        let mut blurred: Option<PathBuf> = None;

        if cli_opts.len() > 7 {
            panic!("Too many arguments");
        }

        cli_opts.next();

        while let Some(arg) = cli_opts.next() {
            match arg.as_str() {
                "--radius" | "-r" => {
                    radius = cli_opts
                        .next()
                        .expect("Expected a value after --radius")
                        .parse::<u8>()
                        .expect("Expected a number after --radius");
                }
                "--sigma" | "-s" => {
                    sigma = cli_opts
                        .next()
                        .expect("Expected a value after --sigma")
                        .parse::<f64>()
                        .expect("Expected a number after --sigma");
                }
                _ => match original {
                    Some(_) => blurred = Some(PathBuf::from(arg)),
                    None => original = Some(PathBuf::from(arg)),
                },
            }
        }

        if original.is_none() {
            panic!("Expected an original image");
        }

        if blurred.is_none() {
            let mut blurred_path: PathBuf = original.clone().unwrap();

            let blurred_fname = format!(
                "{}_blurred_{}x{}.{}",
                blurred_path
                    .file_stem()
                    .expect("Expected a filename")
                    .to_str()
                    .expect("Expected a string"),
                radius,
                sigma,
                blurred_path
                    .extension()
                    .expect("Expected an extension")
                    .to_str()
                    .expect("Expected a string")
            );

            blurred_path.set_file_name(blurred_fname);

            blurred = Some(blurred_path);
        }

        Opts {
            radius,
            sigma,
            original: original.unwrap(),
            blurred: blurred.unwrap(),
        }
    }
}

fn gaussian(x: i32, y: i32, sigma: f64) -> f64 {
    (-(x.pow(2) + y.pow(2)) as f64 / (2.0 * sigma * sigma)).exp()
        / (2.0 * std::f64::consts::PI * sigma * sigma)
}

fn get_gaussian_matrix(radius: u8, sigma: f64) -> Grid<f64> {
    let width = radius * 2 + 1;

    let mut matrix = Grid::new(width as usize, width as usize);

    for x in 0..width {
        for y in 0..width {
            let el = matrix
                .get_mut(x, y)
                .expect("get_gaussian_matrix: Index out of bounds");

            *el = gaussian(x as i32 - radius as i32, y as i32 - radius as i32, sigma);
        }
    }

    matrix
}

fn calculate_new_pixel(x: i32, y: i32, matrix: &Grid<f64>, original_img: &RgbImage) -> Rgb<u8> {
    let mut r = 0.0;
    let mut g = 0.0;
    let mut b = 0.0;
    let mut total = 0.0;

    let radius = matrix.rows() as i32 / 2;

    for i in 0..(matrix.rows() as i32) {
        for k in 0..(matrix.cols() as i32) {
            let x = x + i - radius;
            let y = y + k - radius;

            if x < 0
                || y < 0
                || x >= original_img.width() as i32
                || y >= original_img.height() as i32
            {
                continue;
            }

            let pixel = original_img.get_pixel(x as u32, y as u32);

            let el = matrix.get(i, k).expect("Index out of bounds");

            r += pixel[0] as f64 * el;
            g += pixel[1] as f64 * el;
            b += pixel[2] as f64 * el;
            total += el;
        }
    }

    let r = r / total;
    let g = g / total;
    let b = b / total;

    Rgb::from([r as u8, g as u8, b as u8])
}

pub fn blur_async(radius: u8, sigma: f64, original_img: RgbImage) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let width = original_img.width();
    let height = original_img.height();

    println!("Image dimensions: {}x{}", width, height);

    let n_calculations = width as u128 * height as u128;
    let m_size = (radius as u128 * 2 + 1).pow(2);

    println!("Number of caculations: {}", n_calculations * m_size);

    let mut img_buf = ImageBuffer::new(width, height);
    let m = Arc::new(get_gaussian_matrix(radius, sigma));

    let (tx, rx) = mpsc::channel();
    let img = Arc::new(original_img);
    let n_threads = 10;

    let pool = ThreadPool::new(n_threads);

    for (x, y, _) in img_buf.enumerate_pixels() {
        let _m = Arc::clone(&m);
        let _img = Arc::clone(&img);
        let _tx = tx.clone();

        pool.execute(Box::new(move || {
            let new_pixel = calculate_new_pixel(x as i32, y as i32, &_m, &_img);
            _tx.send((x, y, new_pixel)).unwrap();
        }))
    }

    let mut counter = 0;

    let mut last = 0;

    while let Ok(res) = rx.recv() {
        let (x, y, pix) = res;

        let pixel = img_buf.get_pixel_mut(x, y);
        *pixel = pix;
        counter += 1;

        if counter == width * height {
            break;
        }

        let percent = counter as u128 * 100 / n_calculations;
        if percent % 10 == 0 && percent != last {
            println!("{}% done", percent);
            last = percent;
        }
    }
    println!("Done!");

    img_buf
}

pub fn blur(radius: u8, sigma: f64, original_img: RgbImage) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    println!(
        "Image dimensions: {}x{}",
        original_img.width(),
        original_img.height()
    );

    let m = get_gaussian_matrix(radius, sigma);

    let mut img_buf = ImageBuffer::new(original_img.width(), original_img.height());

    for (x, y, pixel) in img_buf.enumerate_pixels_mut() {
        *pixel = calculate_new_pixel(x as i32, y as i32, &m, &original_img);
    }

    img_buf
}
