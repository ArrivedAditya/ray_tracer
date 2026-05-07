mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod ray;
mod sphere;
mod vec3;

use crate::{camera::Camera, hittable_list::HittableList, sphere::Sphere, vec3::Point3};

fn main() {
    let aspect_ratio: f32 = 16.0 / 9.0;
    let image_width = 400;
    let sample_per_pixel = 100;
    let max_depth = 50;

    // world
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    let mut cam = Camera::new(aspect_ratio, image_width, sample_per_pixel, max_depth);
    cam.render(&world);
}
