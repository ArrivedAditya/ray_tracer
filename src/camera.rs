use rand::{RngExt, rngs::ThreadRng};

use crate::{
    color::{Color, write_color},
    hittable::Hittable,
    hittable_list::HittableList,
    interval::Interval,
    ray::Ray,
    vec3::{Point3, Vec3, random_unit_vector},
};

use std::io::{BufWriter, stdout};

pub struct Camera {
    pub image_width: i32,
    pub sample_per_pixel: i32,
    pub max_depth: i32,

    // for random generation
    rng: ThreadRng,

    image_height: i32,
    pixel_samples_scale: f32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f32, image_width: i32, sample_per_pixel: i32, max_depth: i32) -> Self {
        let mut image_height = (image_width as f32 / aspect_ratio) as i32;
        if image_height < 0 {
            image_height = 1;
        }

        let rng = rand::rng();

        let pixel_samples_scale = 1.0 / sample_per_pixel as f32;

        Self {
            image_width,
            sample_per_pixel,
            max_depth,
            rng,
            image_height,
            pixel_samples_scale,
            center: Point3::new(0.0, 0.0, 0.0),
            pixel00_loc: Point3::new(0.0, 0.0, 0.0),
            pixel_delta_u: Vec3::new(0.0, 0.0, 0.0),
            pixel_delta_v: Vec3::new(0.0, 0.0, 0.0),
        }
    }
    pub fn render(&mut self, world: &HittableList) {
        self.initialize();

        println!("P3\n{} {}\n255", self.image_width, self.image_height);

        let mut out = BufWriter::new(stdout());
        let intensity = Interval::new(0.0, 0.999);

        for j in 0..self.image_height {
            eprint!("\rScanlines remaining {} ", self.image_height - j);
            for i in 0..self.image_width {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for sample in 0..self.sample_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += self.ray_color(&r, self.max_depth, world);
                }
                write_color(&mut out, &intensity, self.pixel_samples_scale * pixel_color)
                    .expect("Failed to write");
            }
        }
        eprintln!("\rDone                               ")
    }

    fn initialize(&mut self) {
        // camera & viewport
        let focal_length: f32 = 1.0;
        let viewport_height: f32 = 2.0;
        let viewport_width = viewport_height * (self.image_width as f32 / self.image_height as f32);

        self.center = Point3::new(0.0, 0.0, 0.0);

        // calculate horizontal(u) and vertical(v) viewport edges
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        // calculate horizontal(u) and vertical(v) delta vectors form pixel to pixel
        self.pixel_delta_u = viewport_u / self.image_width as f32;
        self.pixel_delta_v = viewport_v / self.image_height as f32;

        // calculate location of upper left pixel
        let viewport_upper_left =
            self.center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }

    fn get_ray(&mut self, i: i32, j: i32) -> Ray {
        // Construct a camera ray originating from the origin and directed at randomly sampled
        // point around the pixel location i, j.

        let offset = self.sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i as f32 + offset.x) * self.pixel_delta_u)
            + ((j as f32 + offset.y) * self.pixel_delta_v);

        let ray_direction = pixel_sample - self.center;

        Ray::new(self.center, ray_direction)
    }

    fn sample_square(&mut self) -> Vec3 {
        // Returns the vector to a random point in the [-.4,-.5]-[+.5,+.5] unit square.
        Vec3::new(
            self.rng.random_range(-0.5..=0.5),
            self.rng.random_range(-0.5..=0.5),
            0.0,
        )
    }

    fn ray_color(&self, r: &Ray, depth: i32, world: &dyn Hittable) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }
        if let Some(rec) = world.hit(r, Interval::new(0.001, f32::INFINITY)) {
            let dir = rec.normal + random_unit_vector();
            // in return statement below 0.7 is cofficient which controls brightness of light
            return 0.7 * self.ray_color(&Ray::new(rec.p, dir), depth - 1, world);
        }
        // unit_direction vector
        let unit_direction = r.dir.unit_vector();
        let a = 0.5 * (unit_direction.y + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }
}
