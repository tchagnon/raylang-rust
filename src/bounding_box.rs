use math::Vec3f;
use ray_tracer::Ray;
use std::f32;

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct BoundingBox {
    pub min: Vec3f,
    pub max: Vec3f,
}

impl BoundingBox {
    /**
     * Determine if the ray intersects the bounding box.
     *
     * For all 3 axes, calculate the intersection distances t1 and t2.
     */
    pub fn intersect(&self, ray: Ray) -> bool {
        let (t1x, t2x) = get_t1_t2(ray.origin.x, ray.direction.x, self.min.x, self.max.x);
        let (t1y, t2y) = get_t1_t2(ray.origin.y, ray.direction.y, self.min.y, self.max.y);
        let (t1z, t2z) = get_t1_t2(ray.origin.z, ray.direction.z, self.min.z, self.max.z);
        let t_near = t1x.max(t1y.max(t1z));
        let t_far  = t2x.min(t2y.min(t2z));
        if (t_near > t_far) || t_far < 0.0 {
            false
        } else {
            true
        }
    }

    pub fn from_vertices(vertices: &Vec<Vec3f>) -> BoundingBox {
        let (min_x, max_x) = vec3f_min_max(vertices, |v| v.x);
        let (min_y, max_y) = vec3f_min_max(vertices, |v| v.y);
        let (min_z, max_z) = vec3f_min_max(vertices, |v| v.z);
        BoundingBox {
            min: Vec3f { x: min_x, y: min_y, z: min_z },
            max: Vec3f { x: max_x, y: max_y, z: max_z },
        }
    }
}

fn get_t1_t2(ray_origin_x: f32, ray_dir_x: f32, min_x: f32, max_x: f32) -> (f32, f32) {
    let t1 = (min_x - ray_origin_x) / ray_dir_x;
    let t2 = (max_x - ray_origin_x) / ray_dir_x;
    if t2 > t1 {
        (t1, t2)
    } else {
        (t2, t1)
    }
}

fn vec3f_min_max<F>(vs: &Vec<Vec3f>, axis_fn: F) -> (f32, f32)
where F: Fn(&Vec3f) -> f32 {
    let xs: Vec<f32> = vs.iter().map(axis_fn).collect();
    let min_x = xs.iter().cloned().fold(f32::MAX, f32::min);
    let max_x = xs.iter().cloned().fold(f32::MIN, f32::max);
    (min_x, max_x)
}

