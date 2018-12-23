use ::materials::{Scatterable};
use ::utils::vec3::Vec3;
use ::utils::ray::Ray;

use super::hitable::{HitRecord, Hitable};

pub struct Sphere {
    center: Vec3,
    radius: f32,
    material: Box<dyn Scatterable>
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Box<dyn Scatterable>) -> Sphere {
        Sphere {
            center,
            radius,
            material
        }
    }
}

impl Hitable for Sphere  {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let origin_center: Vec3 = ray.origin - self.center;
        let a: f32 = ray.direction.dot(&ray.direction);
        let b: f32 = origin_center.dot(&ray.direction);
        let c: f32 = origin_center.dot(&origin_center) - self.radius * self.radius;
        let discriminant: f32 = b * b -  a * c;
        if discriminant > 0.0 {
            let temp: f32 = (-b - (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                return Some(HitRecord {
                    t: temp,
                    p: ray.point_at_parameter(temp),
                    normal: (ray.point_at_parameter(temp) - self.center) / self.radius,
                    material: &self.material
                });
            }

            let temp: f32 = (-b + (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                return Some(HitRecord {
                    t: temp,
                    p: ray.point_at_parameter(temp),
                    normal: (ray.point_at_parameter(temp) - self.center) / self.radius,
                    material: &self.material
                });
            }
        }

        None
    }
}
