use std::sync::Arc;

use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;

pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut hit_anything: Option<HitRecord> = None;
        let mut closest_so_far = ray_t.max;

        for object in &self.objects {
            if let Some(temp_rec) = object.hit(r, Interval::new(ray_t.min, closest_so_far)) {
                closest_so_far = temp_rec.t;
                hit_anything = Some(temp_rec);
            }
        }

        hit_anything
    }
}
