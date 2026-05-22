mod aabb;
mod bvh;
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

use rand::RngExt;

use crate::{
    bvh::BVHNode,
    camera::Camera,
    color::Color,
    hittable::HittablePtr,
    hittable_list::HittableList,
    material::{Dielectric, Lambertain, Metal},
    sphere::Sphere,
    vec3::{Point3, Vec3},
};

fn main() {
    let mut rng = rand::rng();
    let aspect_ratio: f32 = 16.0 / 9.0;
    let image_width = 400;
    let sample_per_pixel = 100;
    let max_depth = 50;
    let vfow = 20.0;
    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let defocus_angle = 0.6;
    let focus_dist = 10.0;

    let mut world = HittableList::new();

    let material_ground = Arc::new(Lambertain::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Arc::new(Sphere::new_static(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        material_ground,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_material = rng.random_range(0.0..=1.0);
            let center = Point3::new(
                a as f32 + 0.9 * rng.random_range(0.0..=1.0),
                0.2,
                b as f32 + 0.9 * rng.random_range(0.0..=1.0),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_material < 0.8 {
                    // diffuse
                    let albedo = Color::random_color(&mut rng, 0.0, 1.0);
                    let sphere_material = Arc::new(Lambertain::new(albedo));
                    let center2 = center + Vec3::new(0.0, rng.random_range(0.0..=0.5), 0.0);
                    world.add(Arc::new(Sphere::new_moving(
                        center,
                        center2,
                        0.2,
                        sphere_material,
                    )));
                } else if choose_material < 0.95 {
                    // metal
                    let albedo = Color::random_color(&mut rng, 0.5, 1.0);
                    let fuzz = rng.random_range(0.0..=0.5);
                    let sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Arc::new(Sphere::new_static(center, 0.2, sphere_material)));
                } else {
                    // glass
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Arc::new(Sphere::new_static(center, 0.2, sphere_material)));
                }
            }
        }
    }
    let material1 = Arc::new(Dielectric::new(1.50));
    world.add(Arc::new(Sphere::new_static(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Arc::new(Lambertain::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::new_static(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::new_static(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    let bvh_world = BVHNode::new(&mut world.objects, &mut rng);
    let mut world_map = HittableList::new();
    world_map.add(Arc::new(bvh_world));

    let mut cam = Camera::new(
        aspect_ratio,
        image_width,
        sample_per_pixel,
        max_depth,
        vfow,
        lookfrom,
        lookat,
        vup,
        defocus_angle,
        focus_dist,
    );
    cam.render(&world, &mut rng);
}
