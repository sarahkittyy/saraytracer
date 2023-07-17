use super::{Color, Ray, RayContact};
use crate::math::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RayScatter {
    pub ray: Ray,
    pub attenuation: Color,
}

pub trait Material {
    fn scatter(&self, ray: Ray, contact: &RayContact) -> Option<RayScatter>;
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Diffuse {
    pub color: Color,
}

impl Material for Diffuse {
    /// returns the scattered ray and its corresponding attenuation
    fn scatter(&self, _ray: Ray, contact: &RayContact) -> Option<RayScatter> {
        let target = contact.point + Vec3::random_in_hemisphere(contact.normal);
        let scatter = RayScatter {
            ray: Ray::new(contact.point, target - contact.point),
            attenuation: self.color,
        };
        Some(scatter)
    }
}

impl From<Color> for Diffuse {
    fn from(color: Color) -> Self {
        Self { color }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Metal {
    pub color: Color,
    pub fuzz: f64,
}

impl Material for Metal {
    fn scatter(&self, ray: Ray, contact: &RayContact) -> Option<RayScatter> {
        let reflected = ray.direction.normalize().reflect(contact.normal);
        if reflected.dot(contact.normal) <= 0.0 {
            // only reflect in the same direction as the normal
            None
        } else {
            Some(RayScatter {
                ray: Ray::new(
                    contact.point,
                    reflected + self.fuzz * Vec3::random_unit_sphere(),
                ),
                attenuation: self.color,
            })
        }
    }
}
