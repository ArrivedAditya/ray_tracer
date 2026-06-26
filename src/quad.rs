use std::sync::Arc;

use fastrand::Rng;

use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable, HittablePtr},
    hittable_list::HittableList,
    interval::Interval,
    material::MaterialType,
    vec3::{Point3, Vec3},
};

pub struct Quad {
    Q: Point3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    bbox: AABB,
    normal: Vec3,
    D: f32,
    mat: MaterialType,
}

impl Quad {
    pub fn new(Q: Point3, u: Vec3, v: Vec3, mat: MaterialType) -> Self {
        let n = u.cross(&v);
        let normal = n.unit_vector();
        let D = normal.dot(&Q);
        let w = n / n.dot(&n);

        let bbox = Self::set_bounding_box(&Q, &u, &v, mat.clone());
        Self {
            Q,
            u,
            v,
            w,
            bbox,
            normal,
            D,
            mat,
        }
    }

    fn set_bounding_box(Q: &Point3, u: &Vec3, v: &Vec3, _mat: MaterialType) -> AABB {
        let bbox_diagonal1 = AABB::new_defined(Q, &(*Q + *u + *v));
        let bbox_diagonal2 = AABB::new_defined(&(*Q + *u), &(*Q + *v));

        AABB::new_box(&bbox_diagonal1, &bbox_diagonal2)
    }
}

impl Hittable for Quad {
    fn hit(
        &self,
        r: &crate::ray::Ray,
        ray_t: Interval,
        rec: &mut HitRecord,
        rng: &mut Rng,
    ) -> bool {
        let denom = self.normal.dot(&r.dir);

        if denom.abs() < 1e-8 {
            return false;
        }

        let t = (self.D - self.normal.dot(&r.origin)) / denom;
        if !ray_t.contains(t) {
            return false;
        }

        let intersection = r.at(t);
        let planar_hitpt = intersection - self.Q;
        let alpha = self.w.dot(&planar_hitpt.cross(&self.v));
        let beta = self.w.dot(&self.u.cross(&planar_hitpt));

        if !is_interior(alpha, beta, rec) {
            return false;
        }

        rec.t = t;
        rec.p = intersection;
        rec.set_face_normal(r, self.normal);
        rec.material = self.mat.clone();

        true
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}

fn is_interior(a: f32, b: f32, rec: &mut HitRecord) -> bool {
    let unit = Interval::new(0.0, 1.0);

    if !unit.contains(a) || !unit.contains(b) {
        return false;
    }

    rec.u = a;
    rec.v = b;

    true
}

pub fn cube(a: Point3, b: Point3, mat: MaterialType) -> HittablePtr {
    let mut sides = HittableList::new();

    // 1. Construct the absolute minimum and maximum corners
    let min = Point3::new(a.x.min(b.x), a.y.min(b.y), a.z.min(b.z));
    let max = Point3::new(a.x.max(b.x), a.y.max(b.y), a.z.max(b.z));

    // 2. Compute the precise scale vectors along each axis
    let dx = Vec3::new(max.x - min.x, 0.0, 0.0);
    let dy = Vec3::new(0.0, max.y - min.y, 0.0);
    let dz = Vec3::new(0.0, 0.0, max.z - min.z);

    // 3. Construct the 6 faces using the exact canonical starting vertices
    // Front face
    sides.add(Arc::new(Quad::new(
        Point3::new(min.x, min.y, max.z),
        dx,
        dy,
        mat.clone(),
    )));

    // Right face
    sides.add(Arc::new(Quad::new(
        Point3::new(max.x, min.y, max.z),
        -dz,
        dy,
        mat.clone(),
    )));

    // Back face
    sides.add(Arc::new(Quad::new(
        Point3::new(max.x, min.y, min.z),
        -dx,
        dy,
        mat.clone(),
    )));

    // Left face
    sides.add(Arc::new(Quad::new(
        Point3::new(min.x, min.y, min.z),
        dz,
        dy,
        mat.clone(),
    )));

    // Top face
    sides.add(Arc::new(Quad::new(
        Point3::new(min.x, max.y, max.z),
        dx,
        -dz,
        mat.clone(),
    )));

    // Bottom face
    sides.add(Arc::new(Quad::new(
        Point3::new(min.x, min.y, min.z),
        dx,
        dz,
        mat,
    )));

    Arc::new(sides)
}
