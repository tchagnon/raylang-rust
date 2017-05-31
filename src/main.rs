#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

extern crate image;

use std::env;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;

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
        println!("Usage: command <scene.json>");
        return;
    }

    let path = Path::new(&args[1]);
    let mut json_file = File::open(path)
        .expect(&format!("Could not open file {:?}", path));
    let mut json = String::new();
    json_file.read_to_string(&mut json).unwrap();

    let scene = Scene::decode_json(&json).prepare();

    scene.render();
    println!("Wrote file {:?}", scene.image);
}
