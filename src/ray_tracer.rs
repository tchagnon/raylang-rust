use color::Color;
use math::{to_radians, Vec3f, Mat4f};
use scene::{Scene, Material};
use std::cmp::Ordering;

pub struct RayTracer<'a> {
    scene: &'a Scene,
}

impl<'a> RayTracer<'a> {
    pub fn new(scene: &Scene) -> RayTracer {
        RayTracer { scene: scene }
    }

    pub fn trace_pixel(&self, x: u32, y: u32) -> Color {
        let (x, y) = (x as f32, y as f32);
        let ref scene = self.scene;
        let ref camera = scene.camera;
        let d = camera.distance;
        let (w, h) = (scene.width as f32, scene.height as f32);
        let theta = to_radians(camera.fov_angle);
        let sj = 2.0 * d * (theta / 2.0).tan(); // image plane width
        let sk = sj * (h / w);                  // image plane height

        let zv = camera.direction.norm();
        let xv = zv.cross(camera.up).norm();
        let yv = xv.cross(zv).norm();

        let origin = camera.location;
        // position of top-left pixel on image plane
        let p00 = origin + zv.scale(d) - xv.scale(sj/2.0) + yv.scale(sk/2.0);
        let d_jk = p00 + xv.scale(sj * x / (w-1.0)) - yv.scale(sk * y / (h-1.0)) - origin;
        let d_jk = d_jk.norm();

        let ray = Ray { origin: origin, direction: d_jk };

        let intersections = scene.objects.intersect(ray, &scene.default_material);
        if intersections.is_empty() {
            scene.background
        } else {
            let min_intersection = intersections.iter().min().unwrap();
            self.get_color(min_intersection)
        }
    }

    pub fn get_color(&self, intx: &Intersection) -> Color {
        let scene = self.scene;
        let ref material = intx.material;
        let light = scene.ambient_light.vec3f.scale(material.k_ambient);
        Color::new(material.color.vec3f.point_mul(light))
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub origin: Vec3f,
    pub direction: Vec3f,
}

impl Ray {
    pub fn transform(&self, m: &Mat4f) -> Ray {
        Ray {
            origin: m.transform_point(self.origin),
            direction: m.transform_direction(self.direction),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Intersection {
    pub distance: f32,
    pub normal: Vec3f,
    pub material: Material,
}

impl Intersection {
    pub fn new(d: f32, n: Vec3f, m: &Material) -> Intersection {
        Intersection {
            distance: d,
            normal: n,
            material: m.clone(),
        }
    }
}

impl Ord for Intersection {
    fn cmp(&self, other: &Self) -> Ordering {
        let d1 = self.distance;
        let d2 = other.distance;
        d1.partial_cmp(&d2)
            .expect(&format!("Unable to compare distances {} and {}", d1, d2))
    }
}

impl PartialOrd for Intersection {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Intersection { }
