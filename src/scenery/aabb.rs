use ::utils::ray::Ray;
use ::utils::vec3::Vec3;

use std::cmp;

pub struct AABB {
    min: Vec3,
    max: Vec3
}

impl AABB {
    pub fn new(min: Vec3, max: Vec3) -> AABB {
        AABB {
            min,
            max
        }
    }

    // TODO: Consider rewriting to Andrew Kensler's proposal
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> bool {
        for a in 0..3 {
            let t0: f32 = cmp::min(
                (self.min.x - ray.origin.x) / ray.direction.x,
                (self.max.x - ray.origin.x) / ray.direction.x
            );
            let t1: f32 = cmp::max(
                (self.min.x - ray.origin.x) / ray.direction.x,
                (self.max.x - ray.origin.x) / ray.direction.x
            );

            // TODO: Rename - not same as input argument
            let tmin = cmp::max(t0, t_min);
            // TODO: Rename - not same as input argument
            let tmax = cmp::min(t1, t_max);

            if tmax <= tmin {
                return false;
            }
        }

        return true
    }

    pub fn surrounding_box(first: AABB, second: AABB) -> AABB {
        let min = Vec3::new(
            cmp::min(first.min.x, second.min.x),
            cmp::min(first.min.y, second.min.y),
            cmp::min(first.min.z, second.min.z),
        );
        let max = Vec3::new(
            cmp::min(first.max.x, second.max.x),
            cmp::min(first.max.y, second.max.y),
            cmp::min(first.max.z, second.max.z),
        );

        AABB::new(min, max)
    }
}
