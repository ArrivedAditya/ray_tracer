use core::f32;
use std::f32::INFINITY;
use std::fs::DirBuilder;
use std::sync::Arc;

use fastrand::Rng;

use crate::aabb::AABB;
use crate::color::Color;
use crate::hittable_list::HittableList;
use crate::interval::Interval;
use crate::material::{Lambertain, MaterialType};
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    // t is distance
    pub t: f32,
    pub u: f32,
    pub v: f32,
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
            u: 0.0,
            v: 0.0,
            material: Arc::new(Lambertain::from_color(Color::new(0.0, 0.0, 0.0))),
            front_face: false,
        }
    }
}

pub type HittablePtr = Arc<dyn Hittable>;

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord, rng: &mut Rng) -> bool;
    fn bounding_box(&self) -> AABB;
}

pub struct Translate {
    object: HittablePtr,
    offset: Vec3,
    bbox: AABB,
}

impl Translate {
    pub fn new(object: HittablePtr, offset: Vec3) -> Self {
        let bbox = object.bounding_box() + offset;

        Self {
            object,
            offset,
            bbox,
        }
    }
}

impl Hittable for Translate {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord, rng: &mut Rng) -> bool {
        let offset_r = Ray::new(r.origin - self.offset, r.dir, r.time);

        if !(self.object.hit(&offset_r, ray_t, rec, rng)) {
            return false;
        }

        rec.p += self.offset;
        true
    }

    fn bounding_box(&self) -> AABB {
        return self.bbox;
    }
}

pub struct RotateY {
    object: HittablePtr,
    sin_theta: f32,
    cos_theta: f32,
    bbox: AABB,
}

impl RotateY {
    pub fn new(object: HittablePtr, angle: f32) -> Self {
        let radians = angle.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let bbox_orgin = object.bounding_box();

        let mut min = Point3::new(f32::INFINITY, f32::INFINITY, f32::INFINITY);
        let mut max = Point3::new(f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f32 * bbox_orgin.x.max + (1 - i) as f32 * bbox_orgin.x.min;
                    let y = j as f32 * bbox_orgin.y.max + (1 - j) as f32 * bbox_orgin.y.min;
                    let z = k as f32 * bbox_orgin.z.max + (1 - k) as f32 * bbox_orgin.z.min;

                    let new_x = cos_theta * x + sin_theta * z;
                    let new_z = -sin_theta * x + cos_theta * z;

                    let tester = Vec3::new(new_x, y, new_z);

                    min.x = min.x.min(tester.x);
                    max.x = max.x.max(tester.x);

                    min.y = min.y.min(tester.y);
                    max.y = max.y.max(tester.y);

                    min.z = min.z.min(tester.z);
                    max.z = max.z.max(tester.z);
                }
            }
        }

        let bbox = AABB::new_defined(&min, &max);

        Self {
            object,
            sin_theta,
            cos_theta,
            bbox,
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord, rng: &mut Rng) -> bool {
        let origin = Point3::new(
            (self.cos_theta * r.origin.x) - (self.sin_theta * r.origin.z),
            r.origin.y,
            -(self.sin_theta * r.origin.x) - (self.cos_theta * r.origin.z),
        );

        let direction = Vec3::new(
            (self.cos_theta * r.dir.x) - (self.sin_theta * r.dir.z),
            r.dir.y,
            -(self.sin_theta * r.dir.x) - (self.cos_theta * r.dir.z),
        );

        let rotated_r = Ray::new(origin, direction, r.time);

        if !(self.object.hit(&rotated_r, ray_t, rec, rng)) {
            return false;
        }

        rec.p = Point3::new(
            (self.cos_theta * rec.p.x) - (self.sin_theta * rec.p.z),
            rec.p.y,
            (self.sin_theta * rec.p.x) - (self.cos_theta * rec.p.z),
        );

        rec.normal = Vec3::new(
            (self.cos_theta * rec.normal.x) - (self.sin_theta * rec.normal.z),
            rec.normal.y,
            (self.sin_theta * rec.normal.x) - (self.cos_theta * rec.normal.z),
        );

        true
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}
