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

    ab_pdet_ac: Vec3f,
    ar_pdet_ac: Vec3f,
    ab_pdet_ar: Vec3f,
    det_t: f32,

    norm_a: Vec3f,
    norm_b: Vec3f,
    norm_c: Vec3f,
}

impl Face {
    /**
      * Intersect the face with a ray.
      */
    pub fn intersect(&self, ray: Ray, material: &Material, shading: Shading) -> Option<Intersection> {
        let d       = ray.direction;
        let det_a   = self.ab_pdet_ac.dot(d);
        let beta    = self.ar_pdet_ac.dot(d) / det_a;
        let gamma   = self.ab_pdet_ar.dot(d) / det_a;
        let t       = self.det_t / det_a;

        if beta >= 0.0 && gamma >= 0.0 && (beta + gamma) <= 1.0 && t >= 0.0 {
            let normal = match shading {
                Shading::Flat => (self.a-self.b).cross(self.a-self.c).norm(),
                Shading::Smooth => Vec3f::zero(),
            };
            Some(Intersection::new(t, normal, material))
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
            ab_pdet_ac: ab_pdet_ac,
            ar_pdet_ac: a_r.partial_determinant(a_c),
            ab_pdet_ar: a_b.partial_determinant(a_r),
            det_t: ab_pdet_ac.dot(a_r),
            .. self.clone()
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
            .filter_map(|f| f.intersect(ray, material, self.shading))
            .collect()
    }
}
