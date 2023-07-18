use super::Normalize;
use rand::prelude::*;
use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub};

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub const X: Self = Self::new(1.0, 0.0, 0.0);
    pub const Y: Self = Self::new(0.0, 1.0, 0.0);
    pub const Z: Self = Self::new(0.0, 0.0, 1.0);
    pub const ZERO: Self = Self::new(0.0, 0.0, 0.0);
    pub const ONE: Self = Self::new(1.0, 1.0, 1.0);

    #[inline(always)]
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    /// vector magnitude
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    /// vector magnitude, squared
    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// vector with components randomized between [0, 1]
    pub fn random() -> Vec3 {
        let mut rng = thread_rng();
        Vec3 {
            x: rng.gen(),
            y: rng.gen(),
            z: rng.gen(),
        }
    }

    /// random point in a unit sphere
    pub fn random_unit_sphere() -> Vec3 {
        Vec3::random_unit() * random::<f64>()
    }

    /// random unit vector
    pub fn random_unit() -> Vec3 {
        let mut rng = thread_rng();
        let mut r = || 2f64 * rng.gen::<f64>() - 1.;
        Vec3 {
            x: r(),
            y: r(),
            z: r(),
        }
        .normalize()
    }

    /// random point in a hemisphere around the given normal
    pub fn random_in_hemisphere(normal: Vec3) -> Vec3 {
        let sphere = Vec3::random_unit_sphere();
        if sphere.dot(normal) > 0.0 {
            // same hemisphere
            sphere
        } else {
            -sphere
        }
    }

    /// random point in a disk in the xy plane
    pub fn random_in_xy_unit_disk() -> Vec3 {
        let mut v = Vec3::random_unit();
        v.z = 0.;
        v
    }

    /// reflect against a surface with the given normal
    pub fn reflect(&self, normal: Vec3) -> Vec3 {
        *self - 2. * self.dot(normal) * normal
    }

    /// refract on a surface
    pub fn refract(&self, normal: Vec3, etai_over_etat: f64) -> Vec3 {
        let uv = *self;
        let cos_theta = (-uv).dot(normal).min(1.0);
        let perp = etai_over_etat * (uv + cos_theta * normal);
        let parallel = -1. * (1.0 - perp.length_squared()).abs().sqrt() * normal;
        perp + parallel
    }

    /// sqrt x, y, and z
    pub fn sqrt(self) -> Self {
        Self {
            x: self.x.sqrt(),
            y: self.y.sqrt(),
            z: self.z.sqrt(),
        }
    }

    /// dot product
    pub fn dot(self, rhs: Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    /// vector cross product
    pub fn cross(self, other: Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    /// is this vec zero (or very close to it)
    pub fn is_zero(&self) -> bool {
        const E: f64 = 1e-8;
        self.x.abs() < E && self.y.abs() < E && self.z.abs() < E
    }

    /// maps [0.0, 1.0] to [0, 255]
    #[inline]
    pub fn into_rgb8_array(self) -> [u8; 3] {
        [
            (self.x * 255.99) as u8,
            (self.y * 255.99) as u8,
            (self.z * 255.99) as u8,
        ]
    }
}

#[test]
fn random_unit_distribution() {
    let mut vecs = vec![];
    for _ in 0..100000 {
        vecs.push(Vec3::random_unit_sphere());
    }
    let mut sum = 0f64;
    let mut vec_sum = Vec3::ZERO;
    for vec in vecs {
        sum += vec.length();
        vec_sum += vec;
    }
    let average = sum / 100000.0;
    let vec_average = vec_sum / 100000.0;
    // average length should be 0.5
    assert!(0.49 <= average && average <= 0.51);
    // average x, y, and z should be 0
    let e = 0.01;
    assert!(-e <= vec_average.x && vec_average.x <= e);
    assert!(-e <= vec_average.y && vec_average.y <= e);
    assert!(-e <= vec_average.z && vec_average.z <= e);
}

impl Normalize for Vec3 {
    /// returns the normalized vector
    fn normalize(&self) -> Self {
        let len = self.length();
        *self / len
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Div<Vec3> for f64 {
    type Output = Vec3;
    fn div(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: rhs.x / self,
            y: rhs.y / self,
            z: rhs.z / self,
        }
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        self * -1.
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}
