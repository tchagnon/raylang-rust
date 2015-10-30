//! Color

use image::Rgb;
use rustc_serialize::Decoder;
use rustc_serialize::Decodable;
use std::convert::AsRef;
use math::Vec3f;
use math::Clamp;

pub type Color = Vec3f;

impl Color {
    pub fn rgb(&self) -> Rgb<u8> {
        let r = (self.x * 255.0).round().clamp(0.0, 255.0) as u8;
        let g = (self.y * 255.0).round().clamp(0.0, 255.0) as u8;
        let b = (self.z * 255.0).round().clamp(0.0, 255.0) as u8;
        Rgb { data: [r, g, b] }
    }
}

pub static BLACK            : Color = Color { x: 0.0, y: 0.0, z: 0.0 };
pub static WHITE            : Color = Color { x: 1.0, y: 1.0, z: 1.0 };
pub static RED              : Color = Color { x: 1.0, y: 0.0, z: 0.0 };
pub static GREEN            : Color = Color { x: 0.0, y: 1.0, z: 0.0 };
pub static BLUE             : Color = Color { x: 0.0, y: 0.0, z: 1.0 };
pub static CYAN             : Color = Color { x: 0.0, y: 1.0, z: 1.0 };
pub static MAGENTA          : Color = Color { x: 1.0, y: 0.0, z: 1.0 };
pub static YELLOW           : Color = Color { x: 1.0, y: 1.0, z: 0.0 };
pub static AZURE            : Color = Color { x: 0.0, y: 0.5, z: 1.0 };
pub static ORANGE           : Color = Color { x: 1.0, y: 0.5, z: 0.0 };
pub static GRAY             : Color = Color { x: 0.5, y: 0.5, z: 0.5 };
pub static BRIGHTORANGE     : Color = Color { x: 1.0, y: 0.8, z: 0.0 };
pub static DARKGREEN        : Color = Color { x: 0.0, y: 0.5, z: 0.0 };
pub static SKYBLUE          : Color = Color { x: 0.530, y: 0.808, z: 0.922 };
pub static BROWN            : Color = Color { x: 0.596, y: 0.463, z: 0.329 };
pub static DARKBROWN        : Color = Color { x: 0.396, y: 0.263, z: 0.129 };
pub static CORNFLOWERBLUE   : Color = Color { x: 0.392, y: 0.584, z: 0.929 };

impl Decodable for Color {
    fn decode<D: Decoder>(d: &mut D) -> Result<Self, D::Error> {
        match try!(d.read_str()).to_lowercase().as_ref() {
            "black"             => Ok(BLACK),
            "white"             => Ok(WHITE),
            "red"               => Ok(RED),
            "green"             => Ok(GREEN),
            "blue"              => Ok(BLUE),
            "cyan"              => Ok(CYAN),
            "magenta"           => Ok(MAGENTA),
            "yellow"            => Ok(YELLOW),
            "azure"             => Ok(AZURE),
            "orange"            => Ok(ORANGE),
            "gray"              => Ok(GRAY),
            "brightorange"      => Ok(BRIGHTORANGE),
            "darkgreen"         => Ok(DARKGREEN),
            "skyblue"           => Ok(SKYBLUE),
            "brown"             => Ok(BROWN),
            "darkbrown"         => Ok(DARKBROWN),
            "cornflowerblue"    => Ok(CORNFLOWERBLUE),
            _ => Err(d.error("unknown color")),
        }
    }
}
