extern crate num;
extern crate mandelbrot;
use std::str::FromStr;
use num::Complex;
use std::fs::File;
use self::mandelbrot::{render, write_image};

/// Parse the string `s` as a coordinate pair, like `"400x600"` or `"1.1,0.7"`
fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        Some(index) => match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
            (Ok(l), Ok(r)) => Some((l, r)),
            _ => None,
        },
        None => None,
    }
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("", ','), None);
}

fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        Some((re, im)) => Some(Complex { re, im }),
        None => None,
    }
}

#[test]
fn test_parse_complex() {
    assert_eq!(
        parse_complex("1.25,-0.625"),
        Some(Complex {
            re: 1.25,
            im: -0.625,
        })
    );
}

fn write_image_to_file(
    filename: &str,
    pixels: &[u8],
    bounds: (usize, usize),
) -> Result<(), std::io::Error> {
    let output = File::create(filename)?;
    write_image(output, pixels, bounds)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 5 {
        eprintln!("Usage: mandelbrot FILE PIXELS UPPERLEFT LOWERRIGHT");
        eprintln!(
            "Example: {} mandel.png 1000x750 -1.20,0.35 -1,0.20",
            args[0]
        );
        std::process::exit(1);
    }

    let bounds = parse_pair(&args[2], 'x').expect("error parsing image dimensions");
    let upper_left = parse_complex(&args[3]).expect("error parsing upper left corner point");
    let lower_right = parse_complex(&args[4]).expect("error parsing lower right corner point");

    // render(&mut pixels, bounds, upper_left, lower_right);
    let pixels = render(bounds, upper_left, lower_right);

    write_image_to_file(&args[1], &pixels, bounds).expect("error writing PNG file");
}
