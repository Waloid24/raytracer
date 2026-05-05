use std::io::Write;

use crate::interval::Interval;
use crate::vec3::Vec3;

pub type Color = Vec3;

fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        linear_component.sqrt()
    } else {
        0.0
    }
}

pub fn write_color<W: Write>(out: &mut W, pixel_color: &Color) {
    let r = linear_to_gamma(pixel_color.x());
    let g = linear_to_gamma(pixel_color.y());
    let b = linear_to_gamma(pixel_color.z());

    // Translate the [0,1] component values to the byte range [0,255].
    let intensity = Interval::new(0.000, 0.999);
    let rbyte = (256.0 * intensity.clamp(r)) as i32;
    let gbyte = (256.0 * intensity.clamp(g)) as i32;
    let bbyte = (256.0 * intensity.clamp(b)) as i32;

    // Write out the pixel color components.
    writeln!(out, "{} {} {}", rbyte, gbyte, bbyte).expect("failed to write color");
}
