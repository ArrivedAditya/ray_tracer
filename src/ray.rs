use crate::vec3::{Point3, Vec3};

#[derive(Clone, Copy)]
pub struct Ray {
    pub origin: Point3,
    pub dir: Vec3,
    pub time: f32,
}

impl Ray {
    pub fn new(origin: Point3, dir: Vec3, time: f32) -> Self {
        Self { origin, dir, time }
    }

    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + t * self.dir
    }

    pub fn axis_number(&self, axis: i32) -> (f32, f32) {
        if axis == 1 {
            (self.origin.y, self.dir.y)
        } else if axis == 2 {
            (self.origin.z, self.dir.z)
        } else {
            (self.origin.x, self.dir.x)
        }
    }
}
