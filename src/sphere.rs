use std::sync::Arc;

use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::MaterialType;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct Sphere {
    center: Point3,
    radius: f32,
    material: MaterialType,
}

impl Sphere {
    pub fn new(center: Point3, radius: f32, material: MaterialType) -> Self {
        Self {
            center,
            // make sure to get +ve radius as its can't be -ve
            radius: radius.max(0.0),
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let oc = self.center - r.origin;
        let a = r.dir.length_squared();
        let h = r.dir.dot(&oc);
        let c = oc.length_squared() - (self.radius * self.radius);

        let disciminant = (h * h) - (a * c);
        if disciminant < 0.0 {
            return None;
        }

        let sqrtd = disciminant.sqrt();

        // find the nearest root that lies in ray_tmin and ray_tmax
        let mut root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h - sqrtd) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }

        let t = root;
        let p = r.at(t);
        let outward_normal = (p - self.center) / self.radius;

        let mut rec = HitRecord {
            t,
            p,
            normal: Vec3::new(0.0, 0.0, 0.0),
            material: Arc::clone(&self.material),
            front_face: false,
        };

        rec.set_face_normal(r, outward_normal);

        Some(rec)
    }
}
