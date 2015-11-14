use rustc_serialize::Decoder;
use rustc_serialize::Decodable;

use math::{Vec3f, Mat4f};

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
                    center: t.transform_point(&c),
                }
            },
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
