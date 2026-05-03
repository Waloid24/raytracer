mod rtweekend;
mod vec3;
mod color;
mod interval;
mod ray;
mod hittable;
mod hittable_list;
mod sphere;
mod camera;

use crate::camera::Camera;
use crate::hittable_list::HittableList;
use crate::rtweekend::*;
use crate::sphere::Sphere;

fn main() {
    // World

    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera

    let mut cam = Camera::default();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;

    cam.render(&world);
}
