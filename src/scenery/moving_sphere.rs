use ::materials::{Scatterable};
use ::scenery::aabb::AABB;
use ::utils::vec3::Vec3;
use ::utils::ray::Ray;

use super::hitable::{HitRecord, Hitable};

// TODO: Is it possible to just extend sphere?
pub struct MovingSphere {
    center_start: Vec3,
    center_end: Vec3,
    shutter_open_time: f32,
    shutter_close_time: f32,
    radius: f32,
    material: Box<dyn Scatterable>
}

impl MovingSphere {
    pub fn new<T: Scatterable + 'static>(
        center_start: Vec3,
        center_end: Vec3,
        shutter_open_time: f32,
        shutter_close_time: f32,
        radius: f32,
        material: T
    ) -> MovingSphere {
        MovingSphere {
            center_start,
            center_end,
            shutter_open_time,
            shutter_close_time,
            radius,
            material: Box::new(material)
        }
    }

    pub fn center(&self, time: f32) -> Vec3 {
        self.center_end +
            ((time - self.shutter_open_time) /
                (self.shutter_close_time - self.shutter_open_time)
            ) * (self.center_end - self.center_start)
    }
}

impl Hitable for MovingSphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let origin_center: Vec3 = ray.origin - self.center(ray.time);
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
                    normal: (ray.point_at_parameter(temp) - self.center(ray.time)) / self.radius,
                    material: &self.material
                });
            }

            let temp: f32 = (-b + (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                return Some(HitRecord {
                    t: temp,
                    p: ray.point_at_parameter(temp),
                    normal: (ray.point_at_parameter(temp) - self.center(ray.time)) / self.radius,
                    material: &self.material
                });
            }
        }

        None
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        let vec3_radius = Vec3::new(
            self.radius,
            self.radius,
            self.radius
        );

        let first = AABB::new(&self.center(t0) - vec3_radius, &self.center(t0) + vec3_radius));
        let last = AABB::new(&self.center(t1) - vec3_radius, &self.center(t1) + vec3_radius));

        Some(AABB::surrounding_box(first, last))
    }
}
