use rustc_serialize::Decoder;
use rustc_serialize::Decodable;

use math::Vec3f;

#[derive(Debug, PartialEq)]
pub enum Primitive {
    Sphere { radius: f32, center: Vec3f },
}

impl Decodable for Primitive {
    fn decode<D: Decoder>(d: &mut D) -> Result<Self, D::Error> {
        match try!(d.read_struct_field("primitive", 0, |d| {
            Ok(try!(d.read_str()))
        })).as_ref() {
            "Sphere" => {
                let radius = try!(d.read_struct_field("radius", 1, |d| { d.read_f32() }));
                let center = try!(d.read_struct_field("center", 1, |d| { Vec3f::decode(d) }));
                Ok(Primitive::Sphere { radius: radius, center: center })
            },
            t@_ => Err(d.error(&format!("unknown primitive {}", t))),
        }
    }
}
