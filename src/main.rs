mod color;
mod hittable;
mod ray;
mod sphere;
mod vec3;

use crate::color::{Color, write_color};
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

// function that detect sphere using quadratic forumla
fn hit_sphere(center: Point3, radius: f32, r: &Ray) -> f32 {
    let oc = center - r.origin;
    let a = r.dir.length_squared();
    let h = r.dir.dot(&oc);
    let c = oc.length_squared() - (radius * radius);
    let disciminant = (h * h) - (a * c);

    if disciminant < 0.0 {
        -1.0
    } else {
        (h - disciminant.sqrt()) / (a)
    }
}

// determines the color of pixel
fn ray_color(r: &Ray) -> Color {
    let t = hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, r);
    if t > 0.0 {
        // calculate normal vector & assume as unit vector
        let n = (r.at(t) - Vec3::new(0.0, 0.0, -1.0)).unit_vector();
        return 0.5 * Color::new(n.x + 1.0, n.y + 1.0, n.z + 1.0);
    }

    // unit_direction vector
    let unit_direction = r.dir.unit_vector();
    let a = 0.5 * (unit_direction.y + 1.0);
    (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    let aspect_ratio: f32 = 16.0 / 9.0;
    let image_width: i32 = 400;

    // calculate the image_height using aspect_ratio
    let mut image_height = (image_width as f32 / aspect_ratio) as i32;
    if image_height < 1 {
        image_height = 1;
    }

    // camera & viewport
    let focal_length: f32 = 1.0;
    let viewport_height: f32 = 2.0;
    let viewport_width = viewport_height * (image_width as f32 / image_height as f32);
    let camera_center = Point3::new(0.0, 0.0, 0.0);

    // calculate horizontal(u) and vertical(v) viewport edges
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    // calculate horizontal(u) and vertical(v) delta vectors form pixel to pixel
    let pixel_delta_u = viewport_u / image_width as f32;
    let pixel_delta_v = viewport_v / image_height as f32;

    // calculate location of upper left pixel
    let viewport_upper_left =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    // render
    println!("P3\n{image_width} {image_height}\n255");

    for j in 0..image_height {
        eprint!("\rScanlines remaining {} ", image_height - j);
        for i in 0..image_width {
            let pixel_center =
                pixel00_loc + (i as f32 * pixel_delta_u) + (j as f32 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;

            let r = Ray::new(camera_center, ray_direction);
            let pixel_color = ray_color(&r);

            let mut out = std::io::stdout();
            write_color(&mut out, pixel_color).expect("Failed to write");
        }
    }
    eprintln!("\rDone                               ")
}
