use super::{Material, Ray};
use crate::math::{Normalize, Vec3};
use std::{ops::Range, sync::Arc};

#[derive(Clone)]
pub struct RayContact {
    pub t: f64,
    pub point: Vec3,
    pub normal: Vec3,
    pub front_face: bool,
    pub material: Arc<dyn Material>,
}

pub trait Shape {
    fn hit(&self, ray: Ray, bounds: Range<f64>) -> Option<RayContact>;
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Arc<dyn Material + Send + Sync + 'static>,
}

impl Sphere {
    /// constructor
    pub fn new<Mat>(center: Vec3, radius: f64, material: Mat) -> Self
    where
        Mat: Material + Send + Sync + 'static,
    {
        Self {
            center,
            radius,
            material: Arc::new(material),
        }
    }
}

impl PartialEq for RayContact {
    fn eq(&self, other: &Self) -> bool {
        self.t == other.t && self.point == other.point && self.normal == other.normal
    }
}

impl PartialOrd for RayContact {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.t.partial_cmp(&other.t)
    }
}

impl Shape for Sphere {
    fn hit(&self, ray: Ray, bounds: Range<f64>) -> Option<RayContact> {
        let otc = ray.origin - self.center;
        // quadratic parameters
        let a = ray.direction.length_squared();
        let half_b = otc.dot(ray.direction);
        let c = otc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0. {
            None
        } else {
            let sqrtd = discriminant.sqrt();

            // compute root that is in bounds
            let mut root = (-half_b - sqrtd) / a;
            if !bounds.contains(&root) {
                root = (-half_b + sqrtd) / a;
                if !bounds.contains(&root) {
                    return None;
                }
            }

            let point = ray.at(root);
            let normal = (point - self.center).normalize();
            let front_face = ray.direction.dot(normal) < 0.;
            RayContact {
                t: root,
                point,
                normal: if front_face { normal } else { -normal },
                front_face,
                material: self.material.clone(),
            }
            .into()
        }
    }
}
