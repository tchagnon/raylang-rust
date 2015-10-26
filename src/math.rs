//! Math module for core vector and matrix operations.

use std::f32::consts;
use std::ops::{Add, Sub};

/// Degrees to radians
pub fn to_radians(x: f32) -> f32 {
    x * consts::PI / 180.0
}

/*
 * 3x1 real vector type
 */
#[derive(Debug, Default, PartialEq)]
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

    pub fn dot(&self, rhs: &Vec3f) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(&self, rhs: &Vec3f) -> Vec3f {
        Vec3f {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    pub fn point_mul(&self, rhs: &Vec3f) -> Vec3f {
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
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn magnitude(&self) -> f32 {
        f32::sqrt(self.magnitude_squared())
    }

    pub fn norm(&self) -> Vec3f {
        self.scale(1.0 / self.magnitude())
    }
}

impl<'a> Add for &'a Vec3f {
    type Output = Vec3f;
    fn add(self, rhs: &'a Vec3f) -> Vec3f {
        Vec3f {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<'a> Sub for &'a Vec3f {
    type Output = Vec3f;
    fn sub(self, rhs: &'a Vec3f) -> Vec3f {
        Vec3f {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
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
        assert_eq!(&u + &v, vec3f(5.0, 7.0, 9.0));
        assert_eq!(&u - &v, vec3f(-3.0, -3.0, -3.0));
        assert_eq!(Vec3f::zero(), vec3f(0.0, 0.0, 0.0));
        assert_eq!(u.cross(&v), vec3f(-3.0, 6.0, -3.0));
        assert_eq!(u.point_mul(&v), vec3f(4.0, 10.0, 18.0));
        assert_eq!(u.dot(&v), 32.0);
        assert_eq!(u.scale(3.0), vec3f(3.0, 6.0, 9.0));
        assert_eq!(u.magnitude_squared(), 14.0);
        assert_eq!(u.magnitude(), 3.7416575);
        assert_eq!(u.norm(), vec3f(0.26726124, 0.5345225, 0.8017837));
    }
}
