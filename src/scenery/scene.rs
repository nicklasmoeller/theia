use ::utils::ray::Ray;
use super::hitable::{HitRecord, Hitable};
use super::sphere::Sphere;

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

    pub fn add_sphere(&mut self, sphere: Sphere) {
        self.hitables.push(Box::new(sphere));
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