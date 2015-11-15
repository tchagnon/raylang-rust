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
mod ray_tracer;
mod scene;

use ray_tracer::RayTracer;
use scene::Scene;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: command <scene.toml>");
        return;
    }

    let path = Path::new(&args[1]);
    let scene = Scene::read(path).prepare();
    let ray_tracer = RayTracer::new(&scene);

    let mut imgbuf = ImageBuffer::new(scene.width, scene.height);
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = ray_tracer.trace_pixel(x, y).rgb();
    }

    let fout = Path::new(&scene.image);
    let _ = imgbuf.save(fout);
    println!("Wrote file {:?}", fout);
}
