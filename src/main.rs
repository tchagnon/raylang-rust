extern crate image;
extern crate toml;
extern crate rustc_serialize;

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

    let chunk_bufs: Vec<_> = handles.into_iter().map(|h| { h.join().unwrap() }).collect();

    let mut imgbuf = ImageBuffer::new(scene.width, scene.height);
    let mut y_off = 0;
    for buf in chunk_bufs {
        for (x, y, pixel) in buf.enumerate_pixels() {
            imgbuf.put_pixel(x, y_off + y, *pixel);
        }
        y_off += buf.height();
    }

    let fout = Path::new(&scene.image);
    let _ = imgbuf.save(fout);
    println!("Wrote file {:?}", fout);
}
