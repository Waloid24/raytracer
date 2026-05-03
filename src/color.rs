use std::io::Write;

use crate::vec3::Vec3;

pub type Color = Vec3;

pub fn write_color<W: Write>(out: &mut W, pixel_color: &Color) {
    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();

    // Translate the [0,1] component values to the byte range [0,255].
    let rbyte = (255.999 * r) as i32;
    let gbyte = (255.999 * g) as i32;
    let bbyte = (255.999 * b) as i32;

    // Write out the pixel color components.
    writeln!(out, "{} {} {}", rbyte, gbyte, bbyte).expect("failed to write color");
}
