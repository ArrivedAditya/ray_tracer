mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod ray;
mod sphere;
mod vec3;

use std::sync::Arc;

use crate::{
    camera::Camera,
    color::Color,
    hittable_list::HittableList,
    material::{Lambertain, Metal},
    sphere::Sphere,
    vec3::Point3,
};

fn main() {
    let mut world = HittableList::new();

    let material_ground = Arc::new(Lambertain::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertain::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Arc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.3));
    let material_right = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

    let aspect_ratio: f32 = 16.0 / 9.0;
    let image_width = 400;
    let sample_per_pixel = 100;
    let max_depth = 50;

    // world
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.2),
        0.5,
        material_center,
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    let mut cam = Camera::new(aspect_ratio, image_width, sample_per_pixel, max_depth);
    cam.render(&world);
}
