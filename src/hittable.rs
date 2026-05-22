use std::sync::Arc;

use crate::aabb::AABB;
use crate::color::Color;
use crate::interval::Interval;
use crate::material::{Lambertain, MaterialType};
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    // t is distance
    pub t: f32,
    pub material: MaterialType,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = r.dir.dot(&outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        }
    }
}

impl Default for HitRecord {
    fn default() -> Self {
        Self {
            p: Point3::default(),
            normal: Vec3::default(),
            t: 0.0,
            // Create a default placeholder material here
            material: Arc::new(Lambertain::new(Color::new(0.0, 0.0, 0.0))),
            front_face: false,
        }
    }
}

pub type HittablePtr = Arc<dyn Hittable>;

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
    fn bounding_box(&self) -> AABB;
}
