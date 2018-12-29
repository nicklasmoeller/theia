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
}