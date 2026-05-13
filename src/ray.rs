use crate::vec3::{Point3, Vec3};

pub struct Ray {
    pub origin: Point3,
    pub dir: Vec3,
    //time: f32,
}

impl Ray {
    pub fn new(origin: Point3, dir: Vec3) -> Self {
        Self { origin, dir }
    }

    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + t * self.dir
    }
}
