use std::ops::Range;

use crate::math::{Normalize, Vec3};

mod camera;
mod material;
mod shape;

pub use camera::*;
pub use material::*;
pub use shape::*;

pub type Color = Vec3;
impl Color {
    pub const WHITE: Self = Self::ONE;
    pub const BLACK: Self = Self::ZERO;
    pub const GRAY: Self = Self::new(0.5, 0.5, 0.5);
    pub const RED: Self = Self::X;
    pub const GREEN: Self = Self::Y;
    pub const BLUE: Self = Self::Z;
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    /// constructor
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    /// computes the position after the ray travels t units in `direction` from `origin`
    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + t * self.direction
    }
}

impl Normalize for Ray {
    /// normalizes the ray direction. origin unaffected
    fn normalize(&self) -> Self {
        Ray {
            origin: self.origin,
            direction: self.direction.normalize(),
        }
    }
}

#[derive(Default)]
pub struct World {
    pub shapes: Vec<Box<dyn Shape + Send + Sync + 'static>>,
}

impl World {
    pub fn new() -> Self {
        Self { shapes: vec![] }
    }

    pub fn insert<T: Shape + Send + Sync + 'static>(&mut self, shape: T) {
        self.shapes.push(Box::new(shape));
    }
}

impl Shape for World {
    fn hit(&self, ray: Ray, bounds: Range<f64>) -> Option<RayContact> {
        self.shapes
            .iter()
            .map(|shape| shape.hit(ray, bounds.clone()))
            .filter_map(|contact| contact)
            .fold(None, |acc, contact| match acc {
                None => Some(contact),
                Some(min) => {
                    if contact < min {
                        Some(contact)
                    } else {
                        Some(min)
                    }
                }
            })
    }
}
