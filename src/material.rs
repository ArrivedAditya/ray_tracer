use rand::{RngExt, rngs::ThreadRng};

use crate::{
    color::Color,
    hittable::HitRecord,
    ray::Ray,
    vec3::{Vec3, random_unit_vector, reflect, refract},
};
use std::sync::Arc;

pub type MaterialType = Arc<dyn Material>;

pub trait Material: Send + Sync {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord, rng: &mut ThreadRng) -> Option<(Color, Ray)>;
}

// Lambertain handles scattering of light using whitnesss(albedo) parameter.
pub struct Lambertain {
    albedo: Color,
}

impl Lambertain {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertain {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord, rng: &mut ThreadRng) -> Option<(Color, Ray)> {
        let mut scatter_direction = rec.normal + random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        let scattered = Ray::new(rec.p, scatter_direction);
        let attenuation = self.albedo;

        Some((attenuation, scattered))
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f32) -> Self {
        Self {
            albedo,
            fuzz: fuzz.min(1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord, rng: &mut ThreadRng) -> Option<(Color, Ray)> {
        let mut reflected = reflect(_r_in.dir, rec.normal);
        reflected = reflected.unit_vector() + (self.fuzz * random_unit_vector());
        let scattered = Ray::new(rec.p, reflected);
        let attenuation = self.albedo;

        if scattered.dir.dot(&rec.normal) > 0.0 {
            Some((attenuation, scattered))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    refraction_index: f32,
}

impl Dielectric {
    pub fn new(refraction_index: f32) -> Self {
        Self { refraction_index }
    }
}

impl Material for Dielectric {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord, rng: &mut ThreadRng) -> Option<(Color, Ray)> {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let ri = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = _r_in.dir.unit_vector();

        let cos_theta = rec.normal.dot(&(-unit_direction));
        let sin_theta = (1.0 - (cos_theta * cos_theta)).sqrt();

        let cannot_refract = ri * sin_theta > 1.0;
        let direction: Vec3;

        if cannot_refract || reflectance(cos_theta, ri) > rng.random_range(0.0..1.0) {
            direction = reflect(unit_direction, rec.normal);
        } else {
            direction = refract(unit_direction, rec.normal, ri);
        }

        let scattered = Ray::new(rec.p, direction);

        Some((attenuation, scattered))
    }
}

// Schlick's Approximation
fn reflectance(cosine: f32, refraction_index: f32) -> f32 {
    let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
    r0 = r0 * r0;

    r0 + (1.0 - r0) * f32::powi(1.0 - cosine, 5)
}
