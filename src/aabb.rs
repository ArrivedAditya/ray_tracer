// Axis-Aligned Bounding Boxes (AABBs)

use crate::{interval::Interval, vec3::Point3};

#[derive(Default, Clone, Copy)]
pub struct AABB {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl AABB {
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        Self { x, y, z }
    }

    pub fn new_defined(a: &Point3, b: &Point3) -> Self {
        Self {
            x: if a.x <= b.x {
                Interval::new(a.x, b.x)
            } else {
                Interval::new(b.x, a.x)
            },
            y: if a.y <= b.y {
                Interval::new(a.y, b.y)
            } else {
                Interval::new(b.y, a.y)
            },
            z: if a.z <= b.z {
                Interval::new(a.z, b.z)
            } else {
                Interval::new(b.z, a.z)
            },
        }
    }

    pub fn new_box(box0: &AABB, box1: &AABB) -> Self {
        Self {
            x: Interval::new_interval(box0.x, box1.x),
            y: Interval::new_interval(box0.y, box1.y),
            z: Interval::new_interval(box0.z, box1.z),
        }
    }

    pub fn axis_interval(&self, n: i32) -> Interval {
        if n == 1 {
            self.y
        } else if n == 2 {
            self.z
        } else {
            self.x
        }
    }

    pub fn hit(&self, r: &crate::ray::Ray, mut ray_t: Interval) -> bool {
        for axis in 0..3 {
            let ax = self.axis_interval(axis);
            // 0: origin on specific axis and 1: direciton on specific axis
            let origin_and_dir = r.axis_number(axis);
            let adinv = 1.0 / origin_and_dir.1;

            let t0 = (ax.min - origin_and_dir.0) * adinv;
            let t1 = (ax.max - origin_and_dir.0) * adinv;

            if t0 < t1 {
                if t0 > ray_t.min {
                    ray_t.min = t0;
                }
                if t1 < ray_t.max {
                    ray_t.max = t1;
                }
            } else {
                if t1 > ray_t.min {
                    ray_t.min = t1;
                }
                if t0 < ray_t.max {
                    ray_t.max = t0;
                }
            }

            if ray_t.max <= ray_t.min {
                return false;
            }
        }
        true
    }

    pub fn longest_axis(&self) -> i32 {
        let x_size = self.x.size();
        let y_size = self.y.size();
        let z_size = self.z.size();
        if x_size > y_size {
            if x_size > z_size {
                return 0;
            } else {
                return 2;
            }
        } else {
            if y_size > z_size {
                return 1;
            } else {
                return 2;
            }
        }
    }

    pub const EMPTY: AABB = AABB {
        x: Interval::EMPTY,
        y: Interval::EMPTY,
        z: Interval::EMPTY,
    };

    pub const UNIVERSAL: AABB = AABB {
        x: Interval::UNIVERSAL,
        y: Interval::UNIVERSAL,
        z: Interval::UNIVERSAL,
    };
}
