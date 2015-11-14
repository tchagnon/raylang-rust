extern crate image;
extern crate toml;
extern crate rustc_serialize;

use std::env;
use std::path::Path;
use image::ImageBuffer;

mod color;
mod math;
mod mesh;
mod primitive;
mod scene;

use color::Color;
use scene::Scene;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: command <scene.toml>");
        return;
    }

    let path = Path::new(&args[1]);
    let scene = Scene::read(path);
    println!("Raw Scene: {:?}", scene);
    let scene = scene.prepare();
    println!("Prepared Scene: {:?}", scene);

    let mut imgbuf = ImageBuffer::new(scene.width, scene.height);
    for (_x, _y, pixel) in imgbuf.enumerate_pixels_mut() {
        let bg = scene.background.vec3f.scale(0.75);
        *pixel = Color::new(bg).rgb();
    }

    let fout = Path::new(&scene.image);
    let _ = imgbuf.save(fout);
}
