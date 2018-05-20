extern crate image;
extern crate num;
extern crate rayon;

use image::png::PNGEncoder;
use image::ColorType;
use num::Complex;
use rayon::prelude::*;
use std::io::Write;


fn pixel_to_point(
    bounds: (usize, usize),
    pixel: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) -> Complex<f64> {
    let (width, height) = (
        lower_right.re - upper_left.re,
        upper_left.im - lower_right.im,
    );
    Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64,
    }
}

#[test]
fn test_pixel_to_point() {
    assert_eq!(
        pixel_to_point(
            (100, 100),
            (25, 75),
            Complex { re: -1.0, im: 1.0 },
            Complex { re: 1.0, im: -1.0 }
        ),
        Complex { re: -0.5, im: -0.5 }
    );
}

pub fn render(bounds: (usize, usize), upper_left: Complex<f64>, lower_right: Complex<f64>) -> Vec<u8> {
    (0..(bounds.0 * bounds.1))
        .into_par_iter()
        .map(|i| (i % bounds.0, i / bounds.0))
        .map(|(column, row)| pixel_to_point(bounds, (column, row), upper_left, lower_right))
        .map(|point| escape_time(point, 255).map_or(0, |time| 255 - time as u8))
        .collect()
}

pub fn write_image<W: Write>(
    writer: W,
    pixels: &[u8],
    bounds: (usize, usize),
) -> Result<(), std::io::Error> {
    let encoder = PNGEncoder::new(writer);
    encoder.encode(
        &pixels,
        bounds.0 as u32,
        bounds.1 as u32,
        ColorType::Gray(8),
    )
}
pub fn escape_time(c: Complex<f64>, limit: u32) -> Option<u32> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        z = z * z + c;
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
    }

    None
}
