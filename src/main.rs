extern crate image;

use std::env;
use std::fs::File;
use std::path::Path;

mod math;
mod mesh;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: command <file.smf> <output.png>");
        return;
    }

    let path = &args[1];
    let mesh = mesh::read_smf(path);
    println!("vertices: {} faces: {}", mesh.vertices.len(), mesh.faces.len());

    let mut imgbuf = image::ImageBuffer::new(512, 512);
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = image::Rgb([0xffu8, 0x00u8, 0xffu8]);
    }

    let fout = Path::new(&args[2]);
    let _ = imgbuf.save(fout);
}
