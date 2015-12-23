extern crate image;
extern crate toml;
extern crate rustc_serialize;

use std::env;
use std::path::Path;

mod color;
mod math;
mod mesh;
mod primitive;
mod ray_tracer;
mod scene;

use scene::Scene;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: command <scene.toml>");
        return;
    }

    let path = Path::new(&args[1]);
    let scene = Scene::read(path).prepare();

    scene.render();
    println!("Wrote file {:?}", scene.image);
}
