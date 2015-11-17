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

    let threads = 4;
    let mut imgbufs: Vec<_> = (0..threads).map(|_| {
        ImageBuffer::new(scene.width, scene.height/threads)
    }).collect();

    for t in (0..threads) {
        for y in (0..scene.height/threads) {
            for x in (0..scene.width) {
                let yy = t * scene.height / threads + y;
                let t = t as usize;
                imgbufs[t].put_pixel(x, y, ray_tracer.trace_pixel(x, yy).rgb());
            }
        }
    }

    let mut imgbuf = ImageBuffer::new(scene.width, scene.height);
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let chunk_height = scene.height/threads;
        let t = (y / chunk_height) as usize;
        *pixel = *imgbufs[t].get_pixel(x, y % chunk_height);
    }

    let fout = Path::new(&scene.image);
    let _ = imgbuf.save(fout);
    println!("Wrote file {:?}", fout);
}
