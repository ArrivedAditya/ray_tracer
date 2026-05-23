use std::f32::consts::PI;
use std::sync::Arc;

use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::MaterialType;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct Sphere {
    center: Ray,
    radius: f32,
    material: MaterialType,
    bbox: AABB,
}

impl Sphere {
    pub fn new_static(center: Point3, radius: f32, material: MaterialType) -> Self {
        let rvec = Point3::new(radius, radius, radius);

        Self {
            center: Ray::new(center, Vec3::new(0.0, 0.0, 0.0), 0.0),
            // make sure to get +ve radius as its can't be -ve
            radius: radius.max(0.0),
            material,
            bbox: AABB::new_defined(&(center - rvec), &(center + rvec)),
        }
    }

    pub fn new_moving(
        center1: Point3,
        center2: Point3,
        radius: f32,
        material: MaterialType,
    ) -> Self {
        let rvec = Point3::new(radius, radius, radius);
        let box1 = AABB::new_defined(&(center1 - rvec), &(center1 + rvec));
        let box2 = AABB::new_defined(&(center2 - rvec), &(center2 + rvec));
        Self {
            center: Ray::new(center1, center2 - center1, 0.0),
            // make sure to get +ve radius as its can't be -ve
            radius: radius.max(0.0),
            material,
            bbox: AABB::new_box(&box1, &box2),
        }
    }

    pub fn get_sphere_uv(p: &Point3) -> (f32, f32) {
        // p: a given point on the sphere of radius one, centered at the origin.
        // u: returned value [0,1] of angle around the Y axis from X=-1.
        // v: returned value [0,1] of angle from Y=-1 to Y=+1.
        //     <1 0 0> yields <0.50 0.50>       <-1  0  0> yields <0.00 0.50>
        //     <0 1 0> yields <0.50 1.00>       < 0 -1  0> yields <0.50 0.00>
        //     <0 0 1> yields <0.25 0.50>       < 0  0 -1> yields <0.75 0.50>
        let theta = (-p.y).acos();
        let phi = f32::atan2(-p.z, p.x) + PI;

        let u = phi / (2.0 * PI);
        let v = theta / PI;

        (u, v)
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let current_center = self.center.at(r.time);
        let oc = current_center - r.origin;
        let a = r.dir.length_squared();
        let h = r.dir.dot(&oc);
        let c = oc.length_squared() - (self.radius * self.radius);

        let disciminant = (h * h) - (a * c);
        if disciminant < 0.0 {
            return false;
        }

        let sqrtd = disciminant.sqrt();

        // find the nearest root that lies in ray_tmin and ray_tmax
        let mut root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h - sqrtd) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);

        rec.normal = Vec3::default();
        rec.material = Arc::clone(&self.material);
        rec.front_face = false;

        let outward_normal = (rec.p - current_center) / self.radius;
        rec.set_face_normal(r, outward_normal);
        let uv = Sphere::get_sphere_uv(&outward_normal);
        rec.u = uv.0;
        rec.v = uv.1;

        true
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}
