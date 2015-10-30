extern crate image;
extern crate toml;
extern crate rustc_serialize;

use std::env;
use std::path::Path;
use image::ImageBuffer;

mod color;
mod math;
mod mesh;
mod scene;

use scene::Scene;
use mesh::Mesh;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: command <scene.toml>");
        return;
    }

    let path = Path::new(&args[1]);
    let scene = Scene::read(path);
    println!("{:?}", scene);

    let mesh = Mesh::read(Path::new(&scene.mesh));
    println!("vertices: {} faces: {}", mesh.vertices.len(), mesh.faces.len());

    let mut imgbuf = ImageBuffer::new(scene.width, scene.height);
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = scene.background.scale(0.75).rgb();
    }

    let fout = Path::new(&scene.image);
    let _ = imgbuf.save(fout);
}
