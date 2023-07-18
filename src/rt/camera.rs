use crate::math::{Normalize, Vec3};
use crate::rt::Ray;

#[derive(Clone, Copy, PartialEq, Debug)]
struct Screen {
    origin: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

pub struct FixedCamera {
    pub eye: Vec3,
    lens_radius: f64,
    uvw: (Vec3, Vec3, Vec3),
    screen: Screen,
}

impl FixedCamera {
    pub fn new(
        eye: Vec3,
        look_at: Vec3,
        up: Vec3,
        aspect_ratio: f64,
        vfov: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Self {
        let h = (vfov.to_radians() / 2.).tan();
        let viewport_height: f64 = 2.0 * h;
        let viewport_width: f64 = viewport_height * aspect_ratio;

        let w = (eye - look_at).normalize();
        let u = up.cross(w).normalize();
        let v = w.cross(u);

        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let origin = eye - horizontal / 2. - vertical / 2. - focus_dist * w;

        let screen = Screen {
            horizontal,
            vertical,
            origin,
        };
        FixedCamera {
            eye,
            lens_radius: aperture / 2.,
            uvw: (u, v, w),
            screen,
        }
    }
}

pub trait Camera {
    fn get_screen_ray(&self, dx: f64, dy: f64) -> Ray;
}

impl Camera for FixedCamera {
    fn get_screen_ray(&self, dx: f64, dy: f64) -> Ray {
        let rd = self.lens_radius * Vec3::random_in_xy_unit_disk();
        let (u, v, _) = self.uvw;
        let offset = u * rd.x + v * rd.y;

        Ray {
            origin: self.eye + offset,
            direction: self.screen.origin
                + (dx * self.screen.horizontal)
                + (dy * self.screen.vertical)
                - self.eye
                - offset,
        }
    }
}
