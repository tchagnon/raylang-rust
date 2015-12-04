//! Mesh module for reading and representing mesh objects

use std::str::FromStr;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;
use math::{Vec3f, Mat4f};
use ray_tracer::{Ray, Intersection};
use scene::Material;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Face {
    a: Vec3f,
    b: Vec3f,
    c: Vec3f,

    det_t: f32,
}

impl Face {
    /**
      * Intersect the face with a ray.
      */
    pub fn intersect(&self, ray: Ray, material: &Material) -> Option<Intersection> {
        let d       = ray.direction;
        let det_a   = self.a.dot(d);
        let beta    = self.b.dot(d) / det_a;
        let gamma   = self.c.dot(d) / det_a;
        let t       = self.det_t / det_a;

        if beta >= 0.0 && gamma >= 0.0 && (beta + gamma) <= 1.0 && t >= 0.0 {
            // TODO calculate face normal
            Some(Intersection::new(t, Vec3f::zero(), material))
        } else {
            None
        }
    }

    /**
      * Transform the face by matrix t and precompute partial determinants for intersection.
      */
    pub fn transform_prepare(&self, t: &Mat4f, origin: &Vec3f) -> Face {
        let a   = t.transform_point(self.a);
        let a_b = t.transform_direction(self.a - self.b);
        let a_c = t.transform_direction(self.a - self.c);
        let a_r = a - *origin;
        let ab_pdet_ac = a_b.partial_determinant(a_c);
        Face {
            a: ab_pdet_ac,
            b: a_r.partial_determinant(a_c),
            c: a_b.partial_determinant(a_r),
            det_t: ab_pdet_ac.dot(a_r),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Shading {
    Smooth,
    Flat,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Mesh {
    pub faces: Vec<Face>,
    pub shading: Shading,
}

impl Mesh {
    pub fn read(path: &Path, shading: Shading) -> Mesh {
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
            .map(|l| Mesh::read_face(&vertices, l))
            .collect();
        Mesh { faces: faces, shading: shading }
    }

    fn read_vertex(s: &String) -> Vec3f {
        let v: Vec<f32> = s.split_whitespace()
            .filter_map(|x| f32::from_str(x).ok())
            .collect();
        Vec3f { x: v[0], y: v[1], z: v[2] }
    }

    fn read_face(vertices: &Vec<Vec3f>, s: &String) -> Face {
        let v: Vec<usize> = s.split_whitespace()
            .filter_map(|x| usize::from_str(x).ok())
            .map(|x| x-1) // SMF indexes from 1
            .collect();
        Face {
            a: vertices[v[0]],
            b: vertices[v[1]],
            c: vertices[v[2]],
            .. Default::default()
        }
    }

    pub fn transform(&self, t: &Mat4f, origin: &Vec3f) -> Self {
        Mesh {
            faces: self.faces.iter()
                .map(|f| f.transform_prepare(t, origin))
                .collect(),
            shading: self.shading,
        }
    }

    pub fn intersect(&self, ray: Ray, material: &Material) -> Vec<Intersection> {
        self.faces.iter()
            .filter_map(|f| f.intersect(ray, material))
            .collect()
    }
}
