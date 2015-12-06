use color::Color;
use math::{to_radians, Vec3f, Mat4f};
use scene::{Scene, Material, Light};
use std::cmp::Ordering;

pub struct RayTracer<'a> {
    scene: &'a Scene,
}

impl<'a> RayTracer<'a> {
    pub fn new(scene: &Scene) -> RayTracer {
        RayTracer { scene: scene }
    }

    pub fn trace_pixel(&self, x: u32, y: u32) -> Color {
        let subsamples = self.scene.subsamples;
        let (x, y) = (x as f32, y as f32);
        let mut v = Vec3f::zero();
        let step = 1.0 / subsamples as f32;
        for i in (0..subsamples) {
            for j in (0..subsamples) {
                let (i, j) = (i as f32, j as f32);
                v = v + self.trace_subpixel(x + i * step, y + j * step).vec3f;
            }
        }
        let avg_v = v.scale(1.0 / (subsamples * subsamples) as f32);
        Color::new(avg_v)
    }

    pub fn trace_subpixel(&self, x: f32, y: f32) -> Color {
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
            self.get_color(&ray, min_intersection)
        }
    }

    pub fn get_color(&self, ray: &Ray, intx: &Intersection) -> Color {
        let scene = self.scene;
        let ref material = intx.material;

        let intx_point = ray.origin + ray.direction.scale(intx.distance);
        let normal = intx.normal;
        let view = ray.direction.scale(-1.0);

        let diff_spec = |light: &Light| -> Vec3f {
            let light_dir   = (light.position - intx_point).norm();
            let reflection  = (normal.scale(normal.dot0(light_dir) * 2.0) - light_dir).norm();
            let diffuse     = material.k_diffuse * normal.dot0(light_dir);
            let specular    = material.k_specular * reflection.dot0(view).powf(material.n_shininess);
            light.color.vec3f.scale(diffuse + specular)
        };

        let ambient = scene.ambient_light.vec3f.scale(material.k_ambient);
        let light = scene.lights.iter().map(diff_spec).fold(ambient, |a, l| a + l);
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
