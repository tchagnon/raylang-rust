//! Math module for core vector and matrix operations.
#![allow(dead_code)]

use std::f32::consts;
use std::ops::{Add, Sub};

/// Degrees to radians
pub fn to_radians(x: f32) -> f32 {
    x * consts::PI / 180.0
}

pub trait Clamp {
    fn clamp(self: Self, lower: Self, upper: Self) -> Self;
}

impl Clamp for f32 {
    fn clamp(self, lower: f32, upper: f32) -> f32 {
        self.min(upper).max(lower)
    }
}

/**
 * 3x1 real vector type
 */
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq)]
pub struct Vec3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3f {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3f {
        Vec3f {x: x, y: y, z: z}
    }

    pub fn zero() -> Vec3f {
        Vec3f::new(0.0, 0.0, 0.0)
    }

    pub fn dot(&self, rhs: Vec3f) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    /** Dot product, clamped to 0 */
    pub fn dot0(&self, rhs: Vec3f) -> f32 {
        self.dot(rhs).max(0.0)
    }

    pub fn cross(&self, rhs: Vec3f) -> Vec3f {
        Vec3f {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    pub fn point_mul(&self, rhs: Vec3f) -> Vec3f {
        Vec3f {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }

    pub fn scale(&self, rhs: f32) -> Vec3f {
        Vec3f {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }

    pub fn magnitude_squared(&self) -> f32 {
        self.dot(*self)
    }

    pub fn magnitude(&self) -> f32 {
        f32::sqrt(self.magnitude_squared())
    }

    pub fn norm(&self) -> Vec3f {
        self.scale(1.0 / self.magnitude())
    }

    /**
      * Compute a partial determinant using self and b as the first two columns
      * of a 3 column matrix.  Returns a vector which can be dot-product
      * multiplied by the third column to give the determinant scalar value.
      */
    pub fn partial_determinant(&self, b: Vec3f) -> Vec3f {
        let a = self;
        Vec3f {
            x: a.y*b.z - a.z*b.y,
            y: a.z*b.x - a.x*b.z,
            z: a.x*b.y - a.y*b.x,
        }
    }
}

impl Add for Vec3f {
    type Output = Vec3f;
    fn add(self, rhs: Vec3f) -> Vec3f {
        Vec3f {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vec3f {
    type Output = Vec3f;
    fn sub(self, rhs: Vec3f) -> Vec3f {
        Vec3f {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

/**
 * 4x1 real vector type
 */
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq)]
pub struct Vec4f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vec4f {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Vec4f {
        Vec4f {x: x, y: y, z: z, w: w}
    }

    pub fn zero() -> Vec4f {
        Vec4f::new(0.0, 0.0, 0.0, 0.0)
    }

    pub fn dot(&self, rhs: Vec4f) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
    }

    pub fn dot3(&self, rhs: Vec3f) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn point_mul(&self, rhs: Vec4f) -> Vec4f {
        Vec4f {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
            w: self.w * rhs.w,
        }
    }

    pub fn scale(&self, rhs: f32) -> Vec4f {
        Vec4f {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }

    pub fn magnitude_squared(&self) -> f32 {
        self.dot(*self)
    }

    pub fn magnitude(&self) -> f32 {
        f32::sqrt(self.magnitude_squared())
    }

    pub fn norm(&self) -> Vec4f {
        self.scale(1.0 / self.magnitude())
    }
}

impl Add for Vec4f {
    type Output = Vec4f;
    fn add(self, rhs: Vec4f) -> Vec4f {
        Vec4f {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl Sub for Vec4f {
    type Output = Vec4f;
    fn sub(self, rhs: Vec4f) -> Vec4f {
        Vec4f {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

/**
 * 4x4 Row Matrix
 */
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq)]
pub struct Mat4f {
    pub r1: Vec4f,
    pub r2: Vec4f,
    pub r3: Vec4f,
    pub r4: Vec4f,
}

impl Mat4f {
    pub fn new(r1: Vec4f, r2: Vec4f, r3: Vec4f, r4: Vec4f) -> Mat4f {
        Mat4f { r1: r1, r2: r2, r3: r3, r4: r4 }
    }

    pub fn identity() -> Mat4f {
        Mat4f::new(
            Vec4f::new(1.0, 0.0, 0.0, 0.0),
            Vec4f::new(0.0, 1.0, 0.0, 0.0),
            Vec4f::new(0.0, 0.0, 1.0, 0.0),
            Vec4f::new(0.0, 0.0, 0.0, 1.0),
        )
    }

    // Construct a translation matrix
    pub fn translate(v: Vec3f) -> Mat4f {
        Mat4f::new(
            Vec4f::new(1.0, 0.0, 0.0, v.x),
            Vec4f::new(0.0, 1.0, 0.0, v.y),
            Vec4f::new(0.0, 0.0, 1.0, v.z),
            Vec4f::new(0.0, 0.0, 0.0, 1.0),
        )
    }

    // Construct a scaling matrix
    pub fn scale(v: Vec3f) -> Mat4f {
        Mat4f::new(
            Vec4f::new(v.x, 0.0, 0.0, 0.0),
            Vec4f::new(0.0, v.y, 0.0, 0.0),
            Vec4f::new(0.0, 0.0, v.z, 0.0),
            Vec4f::new(0.0, 0.0, 0.0, 1.0),
        )
    }

    // Construct a rotation matrix
    pub fn rotate(v: Vec3f, angle: f32) -> Mat4f {
        let c = to_radians(angle).cos();
        let s = to_radians(angle).sin();
        let (x, y, z) = (v.x, v.y, v.z);
        let (x2, y2, z2) = (x.powi(2), y.powi(2), z.powi(2));
        Mat4f::new(
            Vec4f::new(x2+(1.0-x2)*c,     x*y*(1.0-c)-z*s,  x*z*(1.0-c)+y*s,  0.0),
            Vec4f::new(x*y*(1.0-c)+z*s,   y2+(1.0-y2)*c,    y*z*(1.0-c)-x*s,  0.0),
            Vec4f::new(x*z*(1.0-c)-y*s,   y*z*(1.0-c)+x*s,  z2+(1.0-z2)*c,    0.0),
            Vec4f::new(0.0, 0.0, 0.0, 1.0),
        )
    }

    pub fn mv_multiply(&self, v: Vec4f) -> Vec4f {
        Vec4f::new(self.r1.dot(v), self.r2.dot(v), self.r3.dot(v), self.r4.dot(v))
    }

    pub fn transpose(&self) -> Mat4f {
        Mat4f::new(
            Vec4f::new(self.r1.x, self.r2.x, self.r3.x, self.r4.x),
            Vec4f::new(self.r1.y, self.r2.y, self.r3.y, self.r4.y),
            Vec4f::new(self.r1.z, self.r2.z, self.r3.z, self.r4.z),
            Vec4f::new(self.r1.w, self.r2.w, self.r3.w, self.r4.w)
        )
    }

    pub fn mm_multiply(&self, rhs: &Mat4f) -> Mat4f {
        let t = rhs.transpose();
        Mat4f::new(
            self.mv_multiply(t.r1),
            self.mv_multiply(t.r2),
            self.mv_multiply(t.r3),
            self.mv_multiply(t.r4),
        ).transpose()
    }

    pub fn transform_point(&self, point: Vec3f) -> Vec3f {
        let p = Vec4f::new(point.x, point.y, point.z, 1.0);
        let Vec4f { x, y, z, w } = self.mv_multiply(p);
        Vec3f::new(x/w, y/w, z/w)
    }

    pub fn transform_direction(&self, v: Vec3f) -> Vec3f {
        Vec3f::new(self.r1.dot3(v), self.r2.dot3(v), self.r3.dot3(v))
    }
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq)]
pub enum Transform {
    Translate(Vec3f),
    Rotate {
        axis: Vec3f,
        angle: f32,
    },
    Scale(Vec3f),
    Affine(Mat4f),
}

impl Transform {
    pub fn mat4f(&self) -> Mat4f {
        match *self {
            Transform::Translate(v) => Mat4f::translate(v),
            Transform::Rotate { axis, angle } => Mat4f::rotate(axis, angle),
            Transform::Scale(v) => Mat4f::scale(v),
            Transform::Affine(m) => m,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::f32::consts;

    #[test]
    fn test_radians() {
        assert_eq!(to_radians(360.0), consts::PI * 2.0);
    }

    #[test]
    fn test_vec3f() {
        let vec3f = Vec3f::new;
        let u = vec3f(1.0, 2.0, 3.0);
        let v = vec3f(4.0, 5.0, 6.0);
        assert_eq!(u + v, vec3f(5.0, 7.0, 9.0));
        assert_eq!(u - v, vec3f(-3.0, -3.0, -3.0));
        assert_eq!(Vec3f::zero(), vec3f(0.0, 0.0, 0.0));
        assert_eq!(u.cross(v), vec3f(-3.0, 6.0, -3.0));
        assert_eq!(u.point_mul(v), vec3f(4.0, 10.0, 18.0));
        assert_eq!(u.dot(v), 32.0);
        assert_eq!(u.scale(3.0), vec3f(3.0, 6.0, 9.0));
        assert_eq!(u.magnitude_squared(), 14.0);
        assert_eq!(u.magnitude(), 3.7416575);
        assert_eq!(u.norm(), vec3f(0.26726124, 0.5345225, 0.8017837));
    }

    #[test]
    fn test_vec4f() {
        let vec4f = Vec4f::new;
        let u = vec4f(1.0, 2.0, 3.0, 4.0);
        let v = vec4f(5.0, 6.0, 7.0, 8.0);
        assert_eq!(u + v, vec4f(6.0, 8.0, 10.0, 12.0));
        assert_eq!(u - v, vec4f(-4.0, -4.0, -4.0, -4.0));
        assert_eq!(Vec4f::zero(), vec4f(0.0, 0.0, 0.0, 0.0));
        assert_eq!(u.point_mul(v), vec4f(5.0, 12.0, 21.0, 32.0));
        assert_eq!(u.dot(v), 70.0);
        assert_eq!(u.scale(3.0), vec4f(3.0, 6.0, 9.0, 12.0));
        assert_eq!(u.magnitude_squared(), 30.0);
        assert_eq!(u.magnitude(), 5.477225575);
        assert_eq!(u.norm(), vec4f(0.1825741858, 0.3651483717, 0.5477225575, 0.7302967433));
    }

    #[test]
    fn test_mat4f() {
        let mat4f = Mat4f::new;
        let vec4f = Vec4f::new;
        let A = mat4f(
            vec4f(1.0, 2.0, 3.0, 4.0),
            vec4f(5.0, 6.0, 7.0, 8.0),
            vec4f(9.0, 10.0, 11.0, 12.0),
            vec4f(13.0, 14.0, 15.0, 16.0));
        let B = mat4f(
            vec4f(17.0, 18.0, 19.0, 20.0),
            vec4f(21.0, 22.0, 23.0, 24.0),
            vec4f(25.0, 26.0, 27.0, 28.0),
            vec4f(29.0, 30.0, 31.0, 23.0));
        let C = mat4f(
            vec4f(250.0, 260.0, 270.0, 244.0),
            vec4f(618.0, 644.0, 670.0, 624.0),
            vec4f(986.0, 1028.0, 1070.0, 1004.0),
            vec4f(1354.0, 1412.0, 1470.0, 1384.0));
        assert_eq!(A.mm_multiply(&B), C);
    }
}
