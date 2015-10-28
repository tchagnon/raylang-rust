extern crate image;
extern crate toml;
extern crate rustc_serialize;

use std::env;
use std::fs::File;
use std::path::Path;
use image::{ImageBuffer, Rgb};

mod math;
mod mesh;
mod scene;

use scene::Scene;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: command <scene.toml>");
        return;
    }

    let path = Path::new(&args[1]);
    let scene = Scene::read(path);
    println!("{:?}", scene);

    let mesh = mesh::read_smf(&scene.mesh);
    println!("vertices: {} faces: {}", mesh.vertices.len(), mesh.faces.len());

    let mut imgbuf = ImageBuffer::new(scene.width, scene.height);
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = Rgb([0xffu8, 0x00u8, 0xffu8]);
    }

    let fout = Path::new(&scene.image);
    let _ = imgbuf.save(fout);
}
