use ::scenery::hitable::HitRecord;

use ::utils::ray::Ray;
use ::utils::vec3::Vec3;

pub struct Material {
    pub ray: Ray,
    pub attenuation: Vec3
}

pub trait Scatterable: Sync {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<Material>;
}

pub fn random_in_unit_sphere() -> Vec3 {
    let mut point: Vec3;
    while {
        point = 2.0 * Vec3::new(
            rand::random(),
            rand::random(),
            rand::random()
        ) - Vec3::new(1.0, 1.0, 1.0);

        point.squared_length() >= 1.0
    } {}
    point
}

pub mod dielectric;
pub mod lambertian;
pub mod metal;
