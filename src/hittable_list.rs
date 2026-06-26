use fastrand::Rng;

use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable, HittablePtr};
use crate::interval::Interval;
use crate::ray::Ray;

pub struct HittableList {
    pub objects: Vec<HittablePtr>,
    bbox: AABB,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            bbox: AABB::default(),
        }
    }

    pub fn add(&mut self, object: HittablePtr) {
        self.bbox = AABB::new_box(&self.bbox, &object.bounding_box());
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord, rng: &mut Rng) -> bool {
        let mut closest_so_far = ray_t.max;
        let mut hit_anything = false;

        for object in &self.objects {
            if object.hit(r, Interval::new(ray_t.min, closest_so_far), rec, rng) {
                closest_so_far = rec.t;
                hit_anything = true;
            }
        }

        hit_anything
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}
