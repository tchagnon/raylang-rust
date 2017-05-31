//! Color

use image::Rgb;
use math::Vec3f;
use math::Clamp;

#[derive(Clone, Copy, Debug, Deserialize, PartialEq)]
pub enum Color {
    Black,
    White,
    Red,
    Green,
    Blue,
    Cyan,
    Magenta,
    Yellow,
    Azure,
    Orange,
    Gray,
    Brightorange,
    DarkGreen,
    SkyBlue,
    Brown,
    DarkBrown,
    CornflowerBlue,
    Rgb(Vec3f),
}

impl Color {
    pub fn vec3f(&self) -> Vec3f {
        match *self {
            Color::Black            => Vec3f { x: 0.0, y: 0.0, z: 0.0},
            Color::White            => Vec3f { x: 1.0, y: 1.0, z: 1.0},
            Color::Red              => Vec3f { x: 1.0, y: 0.0, z: 0.0},
            Color::Green            => Vec3f { x: 0.0, y: 1.0, z: 0.0},
            Color::Blue             => Vec3f { x: 0.0, y: 0.0, z: 1.0},
            Color::Cyan             => Vec3f { x: 0.0, y: 1.0, z: 1.0 },
            Color::Magenta          => Vec3f { x: 1.0, y: 0.0, z: 1.0 },
            Color::Yellow           => Vec3f { x: 1.0, y: 1.0, z: 0.0 },
            Color::Azure            => Vec3f { x: 0.0, y: 0.5, z: 1.0 },
            Color::Orange           => Vec3f { x: 1.0, y: 0.5, z: 0.0 },
            Color::Gray             => Vec3f { x: 0.5, y: 0.5, z: 0.5 },
            Color::Brightorange     => Vec3f { x: 1.0, y: 0.8, z: 0.0 },
            Color::DarkGreen        => Vec3f { x: 0.0, y: 0.5, z: 0.0 },
            Color::SkyBlue          => Vec3f { x: 0.530, y: 0.808, z: 0.922 },
            Color::Brown            => Vec3f { x: 0.596, y: 0.463, z: 0.329 },
            Color::DarkBrown        => Vec3f { x: 0.396, y: 0.263, z: 0.129 },
            Color::CornflowerBlue   => Vec3f { x: 0.392, y: 0.584, z: 0.929 },
            Color::Rgb(v)           => v,
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
