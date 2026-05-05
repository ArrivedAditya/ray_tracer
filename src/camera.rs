use crate::{
    color::{Color, write_color},
    hittable::Hittable,
    hittable_list::HittableList,
    interval::Interval,
    ray::Ray,
    vec3::{Point3, Vec3},
};

use std::io::{BufWriter, stdout};

pub struct Camera {
    pub aspect_ratio: f32,
    pub image_width: i32,

    image_height: i32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f32, image_width: i32) -> Self {
        let mut image_height = (image_width as f32 / aspect_ratio) as i32;
        if image_height < 1 {
            image_height = 1;
        }

        Self {
            aspect_ratio,
            image_width,
            image_height,
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

        for j in 0..self.image_height {
            eprint!("\rScanlines remaining {} ", self.image_height - j);
            for i in 0..self.image_width {
                let pixel_center = self.pixel00_loc
                    + (i as f32 * self.pixel_delta_u)
                    + (j as f32 * self.pixel_delta_v);
                let ray_direction = pixel_center - self.center;

                let r = Ray::new(self.center, ray_direction);
                let pixel_color = self.ray_color(&r, world);

                write_color(&mut out, pixel_color).expect("Failed to write");
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

    fn ray_color(&self, r: &Ray, world: &dyn Hittable) -> Color {
        if let Some(rec) = world.hit(r, Interval::new(0.0, f32::INFINITY)) {
            return 0.5 * Color::new(rec.normal.x + 1.0, rec.normal.y + 1.0, rec.normal.z + 1.0);
        }
        // unit_direction vector
        let unit_direction = r.dir.unit_vector();
        let a = 0.5 * (unit_direction.y + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }
}
