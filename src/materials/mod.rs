use ::scenery::hitable::HitRecord;

use ::utils::ray::Ray;
use ::utils::vec3::Vec3;

pub struct Material {
    pub ray: Ray,
    pub attenuation: Vec3
}

pub trait Scatterable {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<Material>;
}

pub mod lambertian;
