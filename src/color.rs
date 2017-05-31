//! Color

use image::Rgb;
use std::convert::AsRef;
use math::Vec3f;
use math::Clamp;

#[derive(Clone, Copy, Debug, Deserialize, PartialEq)]
pub enum Color {
    Black,
    White,
    Red,
    Green,
    Blue,
    Rgb(Vec3f),
    //pub vec3f: Vec3f
}

impl Color {
    pub fn vec3f(&self) -> Vec3f {
        match *self {
            Color::Black        => Vec3f { x: 0.0, y: 0.0, z: 0.0},
            Color::White        => Vec3f { x: 1.0, y: 1.0, z: 1.0},
            Color::Red          => Vec3f { x: 1.0, y: 0.0, z: 0.0},
            Color::Green        => Vec3f { x: 0.0, y: 1.0, z: 0.0},
            Color::Blue         => Vec3f { x: 0.0, y: 0.0, z: 1.0},
            Color::Rgb(v)       => v,
        }
    }

    pub fn rgb(&self) -> Rgb<u8> {
        let vec3f = self.vec3f();
        let r = (vec3f.x * 255.0).round().clamp(0.0, 255.0) as u8;
        let g = (vec3f.y * 255.0).round().clamp(0.0, 255.0) as u8;
        let b = (vec3f.z * 255.0).round().clamp(0.0, 255.0) as u8;
        Rgb { data: [r, g, b] }
    }
}

impl Default for Color {
    fn default() -> Self {
        Color::Black
    }
}

/*
pub static BLACK            : Color = Color { vec3f: Vec3f { x: 0.0, y: 0.0, z: 0.0 }};
pub static WHITE            : Color = Color { vec3f: Vec3f { x: 1.0, y: 1.0, z: 1.0 }};
pub static RED              : Color = Color { vec3f: Vec3f { x: 1.0, y: 0.0, z: 0.0 }};
pub static GREEN            : Color = Color { vec3f: Vec3f { x: 0.0, y: 1.0, z: 0.0 }};
pub static BLUE             : Color = Color { vec3f: Vec3f { x: 0.0, y: 0.0, z: 1.0 }};
pub static CYAN             : Color = Color { vec3f: Vec3f { x: 0.0, y: 1.0, z: 1.0 }};
pub static MAGENTA          : Color = Color { vec3f: Vec3f { x: 1.0, y: 0.0, z: 1.0 }};
pub static YELLOW           : Color = Color { vec3f: Vec3f { x: 1.0, y: 1.0, z: 0.0 }};
pub static AZURE            : Color = Color { vec3f: Vec3f { x: 0.0, y: 0.5, z: 1.0 }};
pub static ORANGE           : Color = Color { vec3f: Vec3f { x: 1.0, y: 0.5, z: 0.0 }};
pub static GRAY             : Color = Color { vec3f: Vec3f { x: 0.5, y: 0.5, z: 0.5 }};
pub static BRIGHTORANGE     : Color = Color { vec3f: Vec3f { x: 1.0, y: 0.8, z: 0.0 }};
pub static DARKGREEN        : Color = Color { vec3f: Vec3f { x: 0.0, y: 0.5, z: 0.0 }};
pub static SKYBLUE          : Color = Color { vec3f: Vec3f { x: 0.530, y: 0.808, z: 0.922 }};
pub static BROWN            : Color = Color { vec3f: Vec3f { x: 0.596, y: 0.463, z: 0.329 }};
pub static DARKBROWN        : Color = Color { vec3f: Vec3f { x: 0.396, y: 0.263, z: 0.129 }};
pub static CORNFLOWERBLUE   : Color = Color { vec3f: Vec3f { x: 0.392, y: 0.584, z: 0.929 }};
*/

/*
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
*/
