use rustc_serialize::Decoder;
use rustc_serialize::Decodable;

use math::{Vec3f, Mat4f};
use ray_tracer::Ray;

#[derive(Debug, Clone, PartialEq)]
pub enum Primitive {
    Sphere { radius: f32, center: Vec3f },
}

impl Primitive {
    pub fn transform(&self, t: &Mat4f) -> Self {
        match *self {
            Primitive::Sphere { radius: r, center: c } => {
                Primitive::Sphere {
                    radius: r * t.r1.x,
                    center: t.transform_point(c),
                }
            },
        }
    }

    pub fn intersect(&self, ray: Ray) -> Vec<f32> {
        match *self {
            Primitive::Sphere { radius, center } => Primitive::intersect_sphere(radius, center, ray),
        }
    }

    fn intersect_sphere(radius: f32, center: Vec3f, ray: Ray) -> Vec<f32> {
        let o_c = ray.origin - center;
        let b = 2.0 * ray.direction.dot(o_c);
        let c = o_c.magnitude_squared() - radius.powi(2);
        let discrim = b.powi(2) - 4.0 * c;
        if discrim < 0.0 {
            return vec![];
        }

        let t0 = (-b - discrim.sqrt()) / 2.0;
        let t1 = (-b + discrim.sqrt()) / 2.0;

        if t0 < 0.0 {
            if t1 < 0.0 {
                vec![]
            } else {
                vec![t1]
            }
        } else {
            vec![t0, t1]
        }
    }
}

impl Decodable for Primitive {
    fn decode<D: Decoder>(d: &mut D) -> Result<Self, D::Error> {
        match try!(d.read_struct_field("primitive", 0, |d| {
            Ok(try!(d.read_str()))
        })).as_ref() {
            "Sphere" => {
                let radius = try!(d.read_struct_field("radius", 0, |d| { d.read_f32() }));
                let center = try!(d.read_struct_field("center", 0, |d| { Vec3f::decode(d) }));
                Ok(Primitive::Sphere { radius: radius, center: center })
            },
            t@_ => Err(d.error(&format!("unknown primitive {}", t))),
        }
    }
}
