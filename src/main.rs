mod aabb;
mod bvh;
mod camera;
mod color;
mod hittable;
mod hittable_list;
mod image;
mod interval;
mod material;
mod perlin;
mod quad;
mod ray;
mod sphere;
mod texture;
mod vec3;

use std::sync::Arc;

use fastrand::Rng;

use crate::{
    bvh::BVHNode,
    camera::Camera,
    color::Color,
    hittable_list::HittableList,
    material::{Dielectric, Lambertain, Metal},
    quad::Quad,
    sphere::Sphere,
    texture::{CheckerPattern, ImageTexture, NoiseTexture},
    vec3::{Point3, Vec3},
};

fn main() {
    let scene_no = 5;
    match scene_no {
        1 => bouncing_spheres(),
        2 => checkered_sphere(),
        3 => earth(),
        4 => perlin_spheres(),
        5 => quad(),
        _ => panic!("Scene not found"),
    }
}

fn bouncing_spheres() {
    let mut rng = Rng::new();
    let aspect_ratio: f32 = 16.0 / 9.0;
    let image_width = 400;
    let sample_per_pixel = 100;
    let max_depth = 50;
    let vfow = 20.0;
    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::default();
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let defocus_angle = 0.6;
    let focus_dist = 10.0;

    let mut world = HittableList::new();

    let checker = Arc::new(CheckerPattern::form_colors(
        0.32,
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));
    world.add(Arc::new(Sphere::new_static(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertain::new(checker)),
    )));

    let material_ground = Arc::new(Lambertain::from_color(Color::new(0.5, 0.5, 0.5)));
    world.add(Arc::new(Sphere::new_static(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        material_ground,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_material = rng.f32_inclusive();
            let center = Point3::new(
                a as f32 + 0.9 * rng.f32_inclusive(),
                0.2,
                b as f32 + 0.9 * rng.f32_inclusive(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_material < 0.8 {
                    // diffuse
                    let albedo = Color::random_color(&mut rng, 0.0, 1.0);
                    let sphere_material = Arc::new(Lambertain::from_color(albedo));
                    let center2 = center + Vec3::new(0.0, rng.f32_inclusive() * 0.5, 0.0);
                    world.add(Arc::new(Sphere::new_moving(
                        center,
                        center2,
                        0.2,
                        sphere_material,
                    )));
                } else if choose_material < 0.95 {
                    // metal
                    let albedo = Color::random_color(&mut rng, 0.5, 1.0);
                    let fuzz = rng.f32_inclusive() * 0.5;
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

    let material2 = Arc::new(Lambertain::from_color(Color::new(0.4, 0.2, 0.1)));
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
    cam.render(&world_map, &mut rng);
}

fn checkered_sphere() {
    let mut rng = Rng::new();
    let aspect_ratio: f32 = 16.0 / 9.0;
    let image_width = 400;
    let sample_per_pixel = 100;
    let max_depth = 50;
    let vfow = 20.0;
    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::default();
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let defocus_angle = 0.0;
    let focus_dist = 10.0;

    let mut world = HittableList::new();

    let checker = Arc::new(CheckerPattern::form_colors(
        0.32,
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));
    world.add(Arc::new(Sphere::new_static(
        Point3::new(0.0, -10.0, 0.0),
        10.0,
        Arc::new(Lambertain::new(checker.clone())),
    )));

    world.add(Arc::new(Sphere::new_static(
        Point3::new(0.0, 10.0, 0.0),
        10.0,
        Arc::new(Lambertain::new(checker)),
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
    cam.render(&world_map, &mut rng);
}

fn earth() {
    let mut rng = Rng::new();
    let aspect_ratio: f32 = 16.0 / 9.0;
    let image_width = 400;
    let sample_per_pixel = 100;
    let max_depth = 50;
    let vfow = 20.0;
    let lookfrom = Point3::new(0.0, 0.0, 12.0);
    let lookat = Point3::default();
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let defocus_angle = 0.0;
    let focus_dist = 10.0;

    let earth_texture = Arc::new(ImageTexture::new("earthmap.jpg"));
    let earth_surface = Arc::new(Lambertain::new(earth_texture));
    let mut globe = HittableList::new();
    globe.add(Arc::new(Sphere::new_static(
        Point3::default(),
        2.0,
        earth_surface,
    )));

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
    cam.render(&globe, &mut rng);
}

fn perlin_spheres() {
    let mut rng = Rng::new();
    let aspect_ratio: f32 = 16.0 / 9.0;
    let image_width = 400;
    let sample_per_pixel = 100;
    let max_depth = 50;
    let vfow = 20.0;
    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::default();
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let defocus_angle = 0.0;
    let focus_dist = 10.0;

    let mut world = HittableList::new();
    let pertext = Arc::new(NoiseTexture::new(4.0, &mut rng));
    let noisy_material = Arc::new(Lambertain::new(pertext));
    world.add(Arc::new(Sphere::new_static(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        noisy_material.clone(),
    )));

    world.add(Arc::new(Sphere::new_static(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        noisy_material,
    )));

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

fn quad() {
    let mut rng = Rng::new();
    let aspect_ratio: f32 = 16.0 / 9.0;
    let image_width = 400;
    let sample_per_pixel = 100;
    let max_depth = 50;
    let vfow = 80.0;
    let lookfrom = Point3::new(0.0, 0.0, 9.0);
    let lookat = Point3::default();
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let defocus_angle = 0.0;
    let focus_dist = 10.0;

    let mut world = HittableList::new();

    let left_red = Arc::new(Lambertain::from_color(Color::new(1.0, 0.2, 0.2)));
    let back_green = Arc::new(Lambertain::from_color(Color::new(0.2, 1.0, 0.2)));
    let right_blue = Arc::new(Lambertain::from_color(Color::new(0.2, 0.2, 1.0)));
    let upper_orange = Arc::new(Lambertain::from_color(Color::new(1.0, 0.5, 0.0)));
    let lower_teal = Arc::new(Lambertain::from_color(Color::new(0.2, 0.8, 0.8)));

    world.add(Arc::new(Quad::new(
        Point3::new(-3.0, -2.0, 5.0),
        Vec3::new(0.0, 0.0, -4.0),
        Vec3::new(0.0, 4.0, 0.0),
        left_red,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(-2.0, -2.0, 0.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 4.0, 0.0),
        back_green,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(3.0, -2.0, 1.0),
        Vec3::new(0.0, 0.0, 4.0),
        Vec3::new(0.0, 4.0, 0.0),
        right_blue,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(-2.0, 3.0, 1.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 4.0),
        upper_orange,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(-2.0, -3.0, 1.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 4.0),
        lower_teal,
    )));

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
