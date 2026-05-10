use crate::{
    color::Color,
    hittable::HitRecord,
    ray::Ray,
    vec3::{random_unit_vector, reflect},
};
use std::sync::Arc;

pub type MaterialType = Arc<dyn Material>;

pub trait Material: Send + Sync {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
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
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
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
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
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
