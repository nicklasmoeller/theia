use ::scenery::aabb::AABB;
use ::utils::ray::Ray;
use super::hitable::{HitRecord, Hitable};

pub struct Scene {
    hitables: Vec<Box<dyn Hitable>>
}

impl Scene {
    pub fn new() -> Scene {
        let hitables: Vec<Box<dyn Hitable>> = Vec::new();
        Scene {
            hitables
        }
    }

    pub fn add_hitable<T: Hitable + 'static>(&mut self, hitable: T) {
        self.hitables.push(Box::new(hitable));
    }
}

impl Hitable for Scene {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.hitables.iter().fold(
            None,
            |last_hit, hitable| {
                hitable.hit(ray, t_min, last_hit.as_ref().map_or(t_max, |hit| hit.t)).or(last_hit)
            }
        )
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        if self.hitables.is_empty() {
            None
        }

        if let Some(bounding_box) = self.hitables[0].bounding_box(t0, t1) {
            self.hitables.iter().skip(1).fold(
            Some(bounding_box),
            |acc, hitable: Hitable| {
                match hitable.bounding_box(t0, t1) {
                    Some(hit) => Some(AABB::surrounding_box(acc.unwrap(), hit)),
                    None => None
                }
            })
        } else {
            None
        }
    }
}