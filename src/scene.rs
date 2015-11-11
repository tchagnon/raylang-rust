//! Scene module for reading scene config from toml

use rustc_serialize::Decoder;
use rustc_serialize::Decodable;
use rustc_serialize::DecoderHelpers;
use std::convert::AsRef;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use toml::{Parser, Value};
use toml::Decoder as TomlDecoder;

use color::Color;
use math::Vec3f;
use mesh::Mesh;
use primitive::Primitive;

#[derive(Debug, RustcDecodable, Default, PartialEq)]
pub struct Scene {
    pub image: String,
    pub width: u32,
    pub height: u32,
    pub mesh: String,
    pub background: Color,
    pub camera: Camera,
    pub objects: ObjectTree,
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
        let mut decoder = TomlDecoder::new(Value::Table(scene));
        Scene::decode(&mut decoder).unwrap()
    }
}

#[derive(Debug, RustcDecodable, Default, PartialEq)]
pub struct Camera {
    pub distance: f32,
    pub fovAngle: f32,
    pub location: Vec3f,
    pub direction: Vec3f,
    pub up: Vec3f,
}

#[derive(Debug, PartialEq)]
pub enum ObjectTree {
    Group(Vec<ObjectTree>),
    Mesh(Mesh),
    Primitive(Primitive),
    // TODO implement
    // Transform(Mat4f, ObjectTree),
}

impl Default for ObjectTree {
    fn default() -> ObjectTree {
        ObjectTree::Group(vec![])
    }
}

impl Decodable for ObjectTree {
    fn decode<D: Decoder>(d: &mut D) -> Result<Self, D::Error> {
        match try!(d.read_struct_field("type", 0, |d| {
            Ok(try!(d.read_str()))
        })).as_ref() {
            "Group" => Ok(ObjectTree::Group(try!(d.read_struct_field("items", 1, |d| {
                Ok(try!(d.read_to_vec(|d| {
                    Ok(try!(ObjectTree::decode(d)))
                })))
            })))),
            "Mesh" => Ok(ObjectTree::Mesh(try!(d.read_struct_field("mesh", 1, |d| {
                let model_path = try!(d.read_str());
                Ok(Mesh::read(Path::new(&model_path)))
            })))),
            "Primitive" => Ok(ObjectTree::Primitive(try!(Primitive::decode(d)))),
            // TODO implement
            "Transform" => Ok(ObjectTree::Group(vec![])),
            t@_ => Err(d.error(&format!("unknown object type {}", t))),
        }
    }
}
