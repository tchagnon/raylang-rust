//! Scene module for reading scene config from toml

use rustc_serialize::Decodable;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use toml::{Parser, Decoder, Value};

#[derive(Debug, RustcDecodable, Default, PartialEq)]
pub struct Scene {
    pub image: String,
    pub width: u32,
    pub height: u32,
    pub mesh: String,
}

impl Scene {
    pub fn read(path: &Path) -> Scene {
        let mut toml_file = match File::open(path) {
            Ok(file) => file,
            Err(why) => panic!("Could not open \"{:?}\": {}", path, why),
        };
        let mut toml = String::new();
        toml_file.read_to_string(&mut toml).unwrap();

        let scene = Parser::new(&toml).parse().unwrap();
        let mut decoder = Decoder::new(Value::Table(scene));
        Scene::decode(&mut decoder).unwrap()
    }
}
