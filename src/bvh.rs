use rand::{RngExt, rngs::ThreadRng};
use std::{cmp::Ordering, sync::Arc};

use crate::{aabb::AABB, hittable::Hittable, interval::Interval};

struct BVHNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    bbox: AABB,
}

impl BVHNode {
    pub fn new(objects: &mut [Arc<dyn Hittable>], rng: &mut ThreadRng) -> Self {
        let axis = rng.random_range(0..=2);
        let comparator = match axis {
            0 => box_x_compare,
            1 => box_y_compare,
            _ => box_z_compare,
        };
        let object_span = objects.len();
        let left: Arc<dyn Hittable>;
        let right: Arc<dyn Hittable>;

        if object_span == 1 {
            left = Arc::clone(&objects[0]);
            right = Arc::clone(&objects[0]);
        } else if object_span == 2 {
            if comparator(&objects[0], &objects[1]) == Ordering::Greater {
                left = Arc::clone(&objects[1]);
                right = Arc::clone(&objects[0]);
            } else {
                left = Arc::clone(&objects[0]);
                right = Arc::clone(&objects[1]);
            }
        } else {
            objects.sort_by(comparator);
            let mid = object_span / 2;

            let (left_chunk, right_chunk) = objects.split_at_mut(mid);

            left = Arc::new(BVHNode::new(left_chunk, rng));
            right = Arc::new(BVHNode::new(right_chunk, rng));
        }

        let bbox = AABB::new_box(&left.bounding_box(), &right.bounding_box());
        Self { left, right, bbox }
    }
}

pub fn box_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>, axis_index: i32) -> Ordering {
    let a_axis_min = a.bounding_box().axis_interval(axis_index).min;
    let b_axis_min = b.bounding_box().axis_interval(axis_index).min;

    a_axis_min
        .partial_cmp(&b_axis_min)
        .unwrap_or(Ordering::Equal)
}
pub fn box_x_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
    box_compare(a, b, 0)
}
pub fn box_y_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
    box_compare(a, b, 1)
}
pub fn box_z_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
    box_compare(a, b, 2)
}

impl Hittable for BVHNode {
    fn hit(
        &self,
        r: &crate::ray::Ray,
        ray_t: crate::interval::Interval,
        rec: &mut crate::hittable::HitRecord,
    ) -> bool {
        if !(self.bbox.hit(r, ray_t)) {
            return false;
        } else {
            let hit_left = self.left.hit(r, ray_t, rec);

            let hit_right = self.right.hit(
                r,
                Interval::new(ray_t.min, if hit_left { rec.t } else { ray_t.max }),
                rec,
            );

            hit_left || hit_right
        }
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}
