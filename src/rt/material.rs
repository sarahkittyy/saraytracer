use super::{Color, Ray, RayContact};
use crate::math::*;
use rand::prelude::*;

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

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Dielectric {
    pub refraction_index: f64,
}

impl Material for Dielectric {
    fn scatter(&self, ray: Ray, contact: &RayContact) -> Option<RayScatter> {
        // schlick's approximation for reflectance
        fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
            let mut r0 = (1. - ref_idx) / (1. + ref_idx);
            r0 *= r0;
            r0 + (1. - r0) * (1. - cosine).powf(5.)
        }

        let refraction_ratio = if contact.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let dir = ray.direction.normalize();
        let cos_theta = (-dir).dot(contact.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let refracted =
            if cannot_refract || reflectance(cos_theta, refraction_ratio) > random::<f64>() {
                // cannot refract at this angle
                dir.reflect(contact.normal)
            } else {
                dir.refract(contact.normal, refraction_ratio)
            };

        Some(RayScatter {
            ray: Ray::new(contact.point, refracted),
            attenuation: Color::WHITE,
        })
    }
}
