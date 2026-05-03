pub const INFINITY: f64 = f64::INFINITY;
pub const PI: f64 = 3.1415926535897932385;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub use crate::color::Color;
pub use crate::interval::Interval;
pub use crate::ray::Ray;
pub use crate::vec3::{dot, unit_vector, Point3, Vec3};
