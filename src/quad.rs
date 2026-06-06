use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
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
    fn hit(&self, r: &crate::ray::Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
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
