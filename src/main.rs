#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate serde_yaml;

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
        println!("Usage: command <scene.json|.yaml>");
        return;
    }

    let filename = &args[1];
    let path = Path::new(filename);
    let mut file = File::open(path)
        .expect(&format!("Could not open file {:?}", path));
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let scene = if filename.ends_with(".yaml") {
        Scene::decode_yaml(&contents)
    } else {
        Scene::decode_json(&contents)
    }.prepare();

    scene.render();
    println!("Wrote file {:?}", scene.image);
}
