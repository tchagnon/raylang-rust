extern crate image;
extern crate toml;
extern crate rustc_serialize;

use std::cmp;
use std::env;
use std::thread;
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

    let threads = scene.threads;
    let handles: Vec<_> = (0..threads).map(|t| {
        let scene = scene.clone();
        thread::spawn(move || {
            let ray_tracer = RayTracer::new(&scene);
            let extra = if t == threads - 1 { scene.height % threads } else { 0 };
            let (h, w) = (scene.height / threads + extra, scene.width);
            let mut imgbuf = ImageBuffer::new(w, h);
            for y in (0..h) {
                for x in (0..w) {
                    let yy = t * h + y;
                    imgbuf.put_pixel(x, y, ray_tracer.trace_pixel(x, yy).rgb());
                }
            }
            imgbuf
        })
    }).collect();

    let imgbufs: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();

    let mut imgbuf = ImageBuffer::new(scene.width, scene.height);
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let chunk_height = scene.height/threads;
        let t = cmp::min(threads-1, y / chunk_height);
        *pixel = *imgbufs[t as usize].get_pixel(x, y - t*chunk_height);
    }

    let fout = Path::new(&scene.image);
    let _ = imgbuf.save(fout);
    println!("Wrote file {:?}", fout);
}
