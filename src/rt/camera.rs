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
    screen: Screen,
}

impl FixedCamera {
    pub fn new(eye: Vec3, look_at: Vec3, up: Vec3, aspect_ratio: f64, fov: f64) -> Self {
        let h = (fov.to_radians() / 2.).tan();
        let viewport_height: f64 = 2.0 * h;
        let viewport_width: f64 = viewport_height * aspect_ratio;

        let w = eye - look_at;
        let dx = up.cross(w).normalize();
        let dy = w.cross(dx).normalize();

        let horizontal = viewport_width * dx;
        let vertical = viewport_height * dy;
        let origin = eye - horizontal / 2. - vertical / 2. - w;

        let screen = Screen {
            horizontal,
            vertical,
            origin,
        };
        FixedCamera { eye, screen }
    }
}

pub trait Camera {
    fn get_screen_ray(&self, dx: f64, dy: f64) -> Ray;
}

impl Camera for FixedCamera {
    fn get_screen_ray(&self, dx: f64, dy: f64) -> Ray {
        Ray {
            origin: self.eye,
            direction: self.screen.origin
                + (dx * self.screen.horizontal)
                + (dy * self.screen.vertical)
                - self.eye,
        }
    }
}
