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
    ai: usize,
    bi: usize,
    ci: usize,

    ab_pdet_ac: Vec3f,
    ar_pdet_ac: Vec3f,
    ab_pdet_ar: Vec3f,
    det_t: f32,
}

impl Face {

    /**
      * Read the face from a string of indexes
      */
    fn read(s: &String) -> Face {
        let v: Vec<usize> = s.split_whitespace()
            .filter_map(|x| usize::from_str(x).ok())
            .map(|x| x-1) // SMF indexes from 1
            .collect();
        Face {
            ai: v[0],
            bi: v[1],
            ci: v[2],
            .. Default::default()
        }
    }

    pub fn normal(&self, vertices: &Vec<Vec3f>) -> Vec3f {
        let a = vertices[self.ai];
        let b = vertices[self.bi];
        let c = vertices[self.ci];
        (a-b).cross(a-c).norm()
    }

    /**
      * Intersect the face with a ray.
      */
    pub fn intersect(&self, ray: Ray, material: &Material, mesh: &Mesh) -> Option<Intersection> {
        let d       = ray.direction;
        let det_a   = self.ab_pdet_ac.dot(d);
        let beta    = self.ar_pdet_ac.dot(d) / det_a;
        let gamma   = self.ab_pdet_ar.dot(d) / det_a;
        let alpha   = 1.0 - beta - gamma;
        let t       = self.det_t / det_a;

        let norm_a  = mesh.vertex_normals[self.ai];
        let norm_b  = mesh.vertex_normals[self.bi];
        let norm_c  = mesh.vertex_normals[self.ci];

        if beta >= 0.0 && gamma >= 0.0 && (beta + gamma) <= 1.0 && t >= 0.0 {
            let normal = match mesh.shading {
                Shading::Flat => self.normal(&mesh.vertices),
                Shading::Smooth =>
                      norm_a.scale(alpha)
                    + norm_b.scale(beta)
                    + norm_c.scale(gamma)
            };
            Some(Intersection::new(t, normal, material))
        } else {
            None
        }
    }

    /**
      * Precompute partial determinants for intersection.
      */
    pub fn prepare(&self, origin: &Vec3f, vertices: &Vec<Vec3f>) -> Face {
        let a = vertices[self.ai];
        let b = vertices[self.bi];
        let c = vertices[self.ci];
        let a_b = a - b;
        let a_c = a - c;
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
    pub vertices: Vec<Vec3f>,
    pub vertex_normals: Vec<Vec3f>,
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
            .map(|l| Face::read(l))
            .collect();
        let vertex_normals = faces.iter()
            .fold(vec![Vec3f::zero(); vertices.len()], |mut vn, f| {
                let n = f.normal(&vertices);
                vn[f.ai] = vn[f.ai] + n;
                vn[f.bi] = vn[f.bi] + n;
                vn[f.ci] = vn[f.ci] + n;
                vn
        });
        let vertex_normals = vertex_normals.iter().map(|v| v.norm()).collect();
        Mesh {
            faces: faces,
            vertices: vertices,
            vertex_normals: vertex_normals,
            shading: shading,
        }
    }

    fn read_vertex(s: &String) -> Vec3f {
        let v: Vec<f32> = s.split_whitespace()
            .filter_map(|x| f32::from_str(x).ok())
            .collect();
        Vec3f { x: v[0], y: v[1], z: v[2] }
    }

    pub fn transform(&self, t: &Mat4f, origin: &Vec3f) -> Self {
        let vertices = self.vertices.iter()
            .map(|&v| t.transform_point(v))
            .collect();
        let faces = self.faces.iter()
            .map(|f| f.prepare(origin, &vertices))
            .collect();
        Mesh {
            faces: faces,
            vertices: vertices,
            vertex_normals: self.vertex_normals.iter()
                .map(|&v| t.transform_direction(v))
                .collect(),
            shading: self.shading,
        }
    }

    pub fn intersect(&self, ray: Ray, material: &Material) -> Vec<Intersection> {
        self.faces.iter()
            .filter_map(|f| f.intersect(ray, material, self))
            .collect()
    }
}
