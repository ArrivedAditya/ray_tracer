use std::{f32::INFINITY, sync::Arc};

use fastrand::Rng;

use crate::{
    color::Color,
    hittable::{HitRecord, Hittable, HittablePtr},
    interval::Interval,
    material::{Isotropic, MaterialType},
    texture::TexturePtr,
    vec3::Vec3,
};

pub struct ConstantMedium {
    boundary: HittablePtr,
    neg_inv_density: f32,
    phase_func: MaterialType,
}

impl ConstantMedium {
    pub fn form_tex(boundary: HittablePtr, density: f32, tex: TexturePtr) -> Self {
        Self {
            boundary,
            neg_inv_density: (-1.0 / density),
            phase_func: Arc::new(Isotropic::form_tex(tex)),
        }
    }

    pub fn form_color(boundary: HittablePtr, density: f32, albedo: Color) -> Self {
        Self {
            boundary,
            neg_inv_density: (-1.0 / density),
            phase_func: Arc::new(Isotropic::from_color(albedo)),
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(
        &self,
        r: &crate::ray::Ray,
        ray_t: crate::interval::Interval,
        rec: &mut crate::hittable::HitRecord,
        rng: &mut Rng,
    ) -> bool {
        let mut rec1 = HitRecord::default();
        let mut rec2 = HitRecord::default();

        if !(self.boundary.hit(r, Interval::UNIVERSAL, &mut rec1, rng)) {
            return false;
        }
        if !(self.boundary.hit(
            r,
            Interval::new(rec1.t + 0.0001, INFINITY as f32),
            &mut rec2,
            rng,
        )) {
            return false;
        }

        if rec1.t < ray_t.min {
            rec1.t = ray_t.min;
        }

        if rec2.t > ray_t.max {
            rec2.t = ray_t.max;
        }

        if rec1.t >= rec2.t {
            return false;
        }

        if rec1.t < 0.0 {
            rec1.t = 0.0;
        }

        let ray_length = r.dir.length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * rng.f32().ln();

        if hit_distance > distance_inside_boundary {
            return false;
        }

        rec.t = rec1.t + (hit_distance / ray_length);
        rec.p = r.at(rec.t);

        rec.normal = Vec3::new(1.0, 0.0, 0.0);
        rec.front_face = true;
        rec.material = self.phase_func.clone();

        true
    }

    fn bounding_box(&self) -> crate::aabb::AABB {
        self.boundary.bounding_box()
    }
}
