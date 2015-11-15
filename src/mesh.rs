//! Mesh module for reading and representing mesh objects

use std::str::FromStr;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;
use math::{Vec3f, Mat4f};
use ray_tracer::Ray;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Face {
    a: i32,
    b: i32,
    c: i32,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Mesh {
    pub vertices: Vec<Vec3f>,
    pub faces: Vec<Face>,
}

impl Mesh {
    pub fn read(path: &Path) -> Mesh {
        let smf_file = match File::open(path) {
            Ok(file) => file,
            Err(why) => panic!("Could not open \"{:?}\": {}", path, why),
        };
        let input = BufReader::new(smf_file);

        let lines   : Vec<String> = input.lines().map(|l| l.unwrap()).collect();
        let vertices: Vec<Vec3f> = lines.iter()
            .filter(|l| l.starts_with("v "))
            .map(Mesh::read_vertex)
            .collect();
        let faces   : Vec<Face> = lines.iter()
            .filter(|l| l.starts_with("f "))
            .map(Mesh::read_face)
            .collect();
        Mesh { vertices: vertices, faces: faces }
    }

    fn read_vertex(s: &String) -> Vec3f {
        let v: Vec<f32> = s.split_whitespace()
            .filter_map(|x| f32::from_str(x).ok())
            .collect();
        Vec3f { x: v[0], y: v[1], z: v[2] }
    }

    fn read_face(s: &String) -> Face {
        let v: Vec<i32> = s.split_whitespace()
            .filter_map(|x| i32::from_str(x).ok())
            .collect();
        Face { a: v[0], b: v[1], c: v[2] }
    }

    pub fn transform(&self, t: &Mat4f) -> Self {
        let vs: Vec<_> = self.vertices.clone().into_iter()
            .map(|v| t.transform_point(v))
            .collect();
        Mesh { vertices: vs, faces: self.faces.clone() }
    }

    pub fn intersect(&self, ray: Ray) -> Vec<f32> {
        vec![]
    }
}
