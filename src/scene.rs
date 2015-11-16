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
use math::{Vec3f, Mat4f};
use mesh::Mesh;
use primitive::Primitive;
use ray_tracer::Ray;

#[derive(Debug, Clone, RustcDecodable, Default, PartialEq)]
pub struct Scene {
    pub image: String,
    pub width: u32,
    pub height: u32,
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

    // Precompute, flatten and transform objects in the scene
    pub fn prepare(&self) -> Scene {
        let new_objects = self.objects.prepare(&Mat4f::identity(), &self.camera.location);
        Scene {
            objects: new_objects,
            .. self.clone()
        }
    }

}

#[derive(Debug, Clone, RustcDecodable, Default, PartialEq)]
pub struct Camera {
    pub distance: f32,
    pub fov_angle: f32,
    pub location: Vec3f,
    pub direction: Vec3f,
    pub up: Vec3f,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ObjectTree {
    Group(Vec<ObjectTree>),
    Mesh(Mesh),
    Primitive(Primitive),
    Transform {
        child: Box<ObjectTree>,
        transform: Mat4f,
    },
}

impl ObjectTree {
    pub fn prepare(&self, t: &Mat4f, origin: &Vec3f) -> ObjectTree {
        match *self {
            ObjectTree::Group(ref objs) => {
                ObjectTree::Group(objs.iter().map({ |o| o.prepare(t, origin) }).collect())
            },
            ObjectTree::Transform { ref child, ref transform } => {
                let new_t = t.mm_multiply(transform);
                child.prepare(&new_t, origin)
            },
            ObjectTree::Primitive(ref p) => ObjectTree::Primitive(p.transform(t)),
            ObjectTree::Mesh(ref m) => ObjectTree::Mesh(m.transform(t, origin)),
        }
    }

    pub fn intersect(&self, ray: Ray) -> Vec<f32> {
        match *self {
            ObjectTree::Group(ref objs) => {
                objs.iter().flat_map({ |o| o.intersect(ray).into_iter() }).collect()
            },
            ObjectTree::Transform { ref child, ref transform } => {
                child.intersect(ray.transform(transform))
            },
            ObjectTree::Primitive(ref p) => p.intersect(ray),
            ObjectTree::Mesh(ref m) => m.intersect(ray),
        }
    }
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
            "Group" => Ok(ObjectTree::Group(try!(d.read_struct_field("items", 0, |d| {
                d.read_to_vec(|d| { ObjectTree::decode(d) })
            })))),
            "Mesh" => Ok(ObjectTree::Mesh(try!(d.read_struct_field("mesh", 0, |d| {
                let model_path = try!(d.read_str());
                Ok(Mesh::read(Path::new(&model_path)))
            })))),
            "Primitive" => Ok(ObjectTree::Primitive(try!(Primitive::decode(d)))),
            "Transform" => {
                let child = try!(d.read_struct_field("child", 0, |d| { ObjectTree::decode(d) }));
                let xform = try!(d.read_struct_field("transform", 0, |d| { Mat4f::decode(d) }));
                Ok(ObjectTree::Transform { child: Box::new(child), transform: xform })
            },
            t@_ => Err(d.error(&format!("unknown object type {}", t))),
        }
    }
}
