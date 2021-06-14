//! Scene module for reading scene config from json

use std::path::Path;
use std::sync::Arc;
use std::thread;
use time::precise_time_s;
use serde_json;
use serde_yaml;
use image::ImageBuffer;

use color::Color;
use math::{Vec3f, Mat4f, Transform};
use mesh::{Mesh, Shading};
use primitive::Primitive;
use ray_tracer::{RayTracer, Ray, Intersection};
use bounding_box::BoundingBox;

#[derive(Debug, Clone, Default, Deserialize, PartialEq)]
pub struct Scene {
    pub image: String,
    pub width: u32,
    pub height: u32,
    pub threads: u32,
    pub subsamples: u32,
    pub bbox_limit: u32,
    pub background: Color,
    pub camera: Camera,
    pub objects: ObjectTree,
    pub lights: Vec<Light>,
    pub default_material: Material,
    pub ambient_light: Color,
}

impl Scene {

    pub fn decode_json(s: &str) -> Scene {
        serde_json::from_str(s).expect("Unable to decode Scene JSON")
    }

    pub fn decode_yaml(s: &str) -> Scene {
        serde_yaml::from_str(s).expect("Unable to decode Scene JSON")
    }

    // Precompute, flatten and transform objects in the scene
    pub fn prepare(&self) -> Scene {
        let t0 = precise_time_s();
        let new_objects = self.objects.prepare(&Mat4f::identity(), &self.camera.location);
        let dissected_objects = new_objects.construct_bvh(self.bbox_limit);
        println!("Prepare time {:.2}s", precise_time_s() - t0);
        Scene {
            objects: dissected_objects,
            .. self.clone()
        }
    }

    pub fn render(&self) {
        let t0 = precise_time_s();
        let scene = Arc::new(self.clone());

        let threads = scene.threads;
        let handles: Vec<_> = (0..threads).map(|t| {
            let scene = scene.clone();
            thread::spawn(move || {
                let ray_tracer = RayTracer::new(&scene);
                let extra = if t == threads - 1 { scene.height % threads } else { 0 };
                let (h, w) = (scene.height / threads + extra, scene.width);
                let mut imgbuf = ImageBuffer::new(w, h);
                for y in 0..h {
                    for x in 0..w {
                        let yy = t * h + y;
                        imgbuf.put_pixel(x, y, ray_tracer.trace_pixel(x, yy).rgb());
                    }
                }
                imgbuf
            })
        }).collect();

        let chunk_bufs: Vec<_> = handles.into_iter().map(|h| { h.join().unwrap() }).collect();

        let mut imgbuf = ImageBuffer::new(scene.width, scene.height);
        let mut y_off = 0;
        for buf in chunk_bufs {
            for (x, y, pixel) in buf.enumerate_pixels() {
                imgbuf.put_pixel(x, y_off + y, *pixel);
            }
            y_off += buf.height();
        }

        let fout = Path::new(&scene.image);
        let _ = imgbuf.save(fout);
        println!("Render time {:.2}s", precise_time_s() - t0);
    }

}

#[derive(Debug, Clone, Default, Deserialize, PartialEq)]
pub struct Camera {
    pub distance: f32,
    pub fov_angle: f32,
    pub location: Vec3f,
    pub direction: Vec3f,
    pub up: Vec3f,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub enum ObjectTree {
    Group(Vec<ObjectTree>),
    Mesh(Mesh),
    LoadMesh {
        file: String,
        shading: Shading,
    },
    Primitive(Primitive),
    Transform {
        child: Box<ObjectTree>,
        transform: Transform,
    },
    Material {
        child: Box<ObjectTree>,
        material: Material,
    },
    BoundingBox {
        child: Box<ObjectTree>,
        bbox: BoundingBox,
    },
}

impl ObjectTree {
    pub fn prepare(&self, t: &Mat4f, origin: &Vec3f) -> ObjectTree {
        match *self {
            ObjectTree::Group(ref objs) => {
                ObjectTree::Group(objs.iter().map(|o| o.prepare(t, origin)).collect())
            },
            ObjectTree::Transform { ref child, ref transform } => {
                let new_t = t.mm_multiply(&transform.mat4f());
                child.prepare(&new_t, origin)
            },
            ObjectTree::Primitive(ref p) => ObjectTree::Primitive(p.transform(t)),
            ObjectTree::LoadMesh { ref file, shading } => {
                ObjectTree::Mesh(Mesh::read(Path::new(file), shading)).prepare(t, origin)
            }
            ObjectTree::Mesh(ref m) => ObjectTree::Mesh(m.transform(t, origin)),
            ObjectTree::Material { ref child, ref material } => {
                ObjectTree::Material {
                    child: Box::new(child.prepare(t, origin)),
                    material: material.clone(),
                }
            },
            ObjectTree::BoundingBox { ref child, ref bbox } => {
                ObjectTree::BoundingBox {
                    child: Box::new(child.prepare(t, origin)),
                    bbox: bbox.clone(),
                }
            },
        }
    }

    pub fn construct_bvh(&self, bbox_limit: u32) -> ObjectTree {
        match *self {
            ObjectTree::Group(ref objs) => {
                ObjectTree::Group(objs.iter().map(|o| o.construct_bvh(bbox_limit)).collect())
            },
            ObjectTree::Transform { ref child, ref transform } => {
                ObjectTree::Transform {
                    child: Box::new(child.construct_bvh(bbox_limit)),
                    transform: transform.clone(),
                }
            },
            ObjectTree::Primitive(ref p) => {
                ObjectTree::BoundingBox {
                    child: Box::new(self.clone()),
                    bbox: p.bounding_box(),
                }
            },
            ObjectTree::Mesh(ref m) => m.dissect(bbox_limit),
            ObjectTree::Material { ref child, ref material } => {
                ObjectTree::Material {
                    child: Box::new(child.construct_bvh(bbox_limit)),
                    material: material.clone(),
                }
            },
            ObjectTree::BoundingBox { .. } => {
                self.clone()
            },
            _ => ObjectTree::default()
        }
    }

    pub fn intersect(&self, ray: Ray, material: &Material) -> Vec<Intersection> {
        match *self {
            ObjectTree::Group(ref objs) => {
                objs.iter().flat_map(|o| o.intersect(ray, material).into_iter()).collect()
            },
            ObjectTree::Transform { ref child, ref transform } => {
                child.intersect(ray.transform(&transform.mat4f()), material)
            },
            ObjectTree::Primitive(ref p) => p.intersect(ray, material),
            ObjectTree::Mesh(ref m) => m.intersect(ray, material),
            ObjectTree::Material { ref child, ref material } => {
                child.intersect(ray, material)
            },
            ObjectTree::BoundingBox { ref child, ref bbox } => {
                if bbox.intersect(ray) {
                    child.intersect(ray, material)
                } else {
                    vec![]
                }
            },
            _ => vec![]
        }
    }
}

impl Default for ObjectTree {
    fn default() -> ObjectTree {
        ObjectTree::Group(vec![])
    }
}

#[derive(Debug, Clone, Default, Deserialize, PartialEq)]
pub struct Light {
    pub color: Color,
    pub intensity: f32,
    pub position: Vec3f,
}

#[derive(Debug, Clone, Default, Deserialize, PartialEq)]
pub struct Material {
    pub k_diffuse: f32,
    pub k_specular: f32,
    pub k_ambient: f32,
    pub n_shininess: f32,
    pub color: Color,
}
