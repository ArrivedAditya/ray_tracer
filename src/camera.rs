use rand::{RngExt, rngs::ThreadRng};

use crate::{
    color::{Color, write_color},
    hittable::Hittable,
    hittable_list::HittableList,
    interval::Interval,
    ray::Ray,
    vec3::{Point3, Vec3, random_in_unit_disk},
};

use std::io::{BufWriter, stdout};

pub struct Camera {
    pub image_width: i32,
    pub sample_per_pixel: i32,
    pub max_depth: i32,
    pub vfow: f32, // vertical angle field of view
    pub lookfrom: Point3,
    pub lookat: Point3,
    pub vup: Vec3, // Camera relative up direction
    pub defocus_angle: f32,
    pub focus_dist: f32,

    image_height: i32,
    pixel_samples_scale: f32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,

    // Camera frame basis Vectors (u,v,w)
    u: Vec3,
    v: Vec3,
    w: Vec3,
}

impl Camera {
    pub fn new(
        aspect_ratio: f32,
        image_width: i32,
        sample_per_pixel: i32,
        max_depth: i32,
        vfow: f32,
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        defocus_angle: f32,
        focus_dist: f32,
    ) -> Self {
        let mut image_height = (image_width as f32 / aspect_ratio) as i32;
        if image_height < 0 {
            image_height = 1;
        }

        let pixel_samples_scale = 1.0 / sample_per_pixel as f32;

        Self {
            image_width,
            sample_per_pixel,
            max_depth,
            image_height,
            pixel_samples_scale,
            defocus_angle,
            focus_dist,
            center: Point3::new(0.0, 0.0, 0.0),
            pixel00_loc: Point3::new(0.0, 0.0, 0.0),
            pixel_delta_u: Vec3::new(0.0, 0.0, 0.0),
            pixel_delta_v: Vec3::new(0.0, 0.0, 0.0),
            defocus_disk_u: Vec3::new(0.0, 0.0, 0.0),
            defocus_disk_v: Vec3::new(0.0, 0.0, 0.0),
            vfow,
            lookfrom,
            lookat,
            vup,
            w: Vec3::new(0.0, 0.0, 0.0),
            u: Vec3::new(0.0, 0.0, 0.0),
            v: Vec3::new(0.0, 0.0, 0.0),
        }
    }
    pub fn render(&mut self, world: &HittableList, rng: &mut ThreadRng) {
        self.initialize();

        println!("P3\n{} {}\n255", self.image_width, self.image_height);

        let mut out = BufWriter::new(stdout());
        let intensity = Interval::new(0.0, 0.999);

        for j in 0..self.image_height {
            eprint!("\rScanlines remaining {} ", self.image_height - j);
            for i in 0..self.image_width {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..self.sample_per_pixel {
                    let r = self.get_ray(i, j, rng);
                    pixel_color += self.ray_color(&r, self.max_depth, world, rng);
                }
                write_color(&mut out, &intensity, self.pixel_samples_scale * pixel_color)
                    .expect("Failed to write");
            }
        }
        eprintln!("\rDone                               ")
    }

    fn initialize(&mut self) {
        self.center = self.lookfrom;

        let temp = self.lookfrom - self.lookat;

        // camera & viewport
        //let focal_length: f32 = temp.length();
        let theta = self.vfow.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height: f32 = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * (self.image_width as f32 / self.image_height as f32);

        // Camera unit basis vectors calc
        self.w = temp.unit_vector();
        self.u = self.vup.cross(&self.w).unit_vector();
        self.v = self.w.cross(&self.u);

        // calculate horizontal(u) and vertical(v) viewport edges
        let viewport_u = viewport_width * self.u;
        let viewport_v = viewport_height * -self.v;

        // calculate horizontal(u) and vertical(v) delta vectors form pixel to pixel
        self.pixel_delta_u = viewport_u / self.image_width as f32;
        self.pixel_delta_v = viewport_v / self.image_height as f32;

        // calculate location of upper left pixel
        let viewport_upper_left =
            self.center - (self.focus_dist * self.w) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

        // camera defocus disk basis vectors calculation
        let defocus_radius = self.focus_dist * (self.defocus_angle / 2.0).to_radians().tan();
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;
    }

    fn get_ray(&mut self, i: i32, j: i32, rng: &mut ThreadRng) -> Ray {
        // Construct a camera ray originating from the origin and directed at randomly sampled
        // point around the pixel location i, j.

        let offset = self.sample_square(rng);

        let pixel_sample = self.pixel00_loc
            + ((i as f32 + offset.x) * self.pixel_delta_u)
            + ((j as f32 + offset.y) * self.pixel_delta_v);

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample(rng)
        };
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn sample_square(&mut self, rng: &mut ThreadRng) -> Vec3 {
        // Returns the vector to a random point in the [-.4,-.5]-[+.5,+.5] unit square.
        Vec3::new(
            rng.random_range(-0.5..=0.5),
            rng.random_range(-0.5..=0.5),
            0.0,
        )
    }

    fn defocus_disk_sample(&self, rng: &mut ThreadRng) -> Point3 {
        let p = random_in_unit_disk(rng);
        self.center + (p.x * self.defocus_disk_u) + (p.y * self.defocus_disk_v)
    }

    fn ray_color(&self, r: &Ray, depth: i32, world: &dyn Hittable, rng: &mut ThreadRng) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }
        if let Some(rec) = world.hit(r, Interval::new(0.001, f32::INFINITY)) {
            if let Some((attenuation, scattered)) = rec.material.scatter(r, &rec, rng) {
                return attenuation * self.ray_color(&scattered, depth - 1, world, rng);
            } else {
                return Color::new(0.0, 0.0, 0.0);
            }
        }
        // unit_direction vector
        let unit_direction = r.dir.unit_vector();
        let a = 0.5 * (unit_direction.y + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }
}
