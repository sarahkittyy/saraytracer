mod vec3;
pub use vec3::*;

pub trait Normalize {
    fn normalize(&self) -> Self;
}
