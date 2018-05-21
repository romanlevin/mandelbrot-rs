#[macro_use]
extern crate serde_derive;
extern crate actix_web;
extern crate mandelbrot;
extern crate num;

use self::mandelbrot::{render, write_image};
use actix_web::{http, server, App, HttpResponse, Path};
use num::Complex;

#[derive(Deserialize)]
struct MandelRequest {
    width: usize,
    height: usize,
    upper_left_re: f64,
    upper_left_im: f64,
    lower_right_re: f64,
    lower_right_im: f64,
}

fn mandel(request: Path<MandelRequest>) -> HttpResponse {
    let upper_left = Complex::new(request.upper_left_re, request.upper_left_im);
    let lower_right = Complex::new(request.lower_right_re, request.lower_right_im);
    let bounds = (request.width, request.height);
    let pixels = render(bounds, upper_left, lower_right);
    let mut image = Vec::<_>::new();
    write_image(&mut image, &pixels, bounds).unwrap();
    HttpResponse::Ok().content_type("image/png").body(image)
}

fn main() {
    server::new(|| {
        App::new().resource("/{width}x{height}_{upper_left_re}_{upper_left_im}_{lower_right_re}_{lower_right_im}.png", |r| {
            r.method(http::Method::GET).with(mandel)
        })
    }).bind("127.0.0.1:3000")
        .expect("Can't bind to port 3000")
        .run();
}
