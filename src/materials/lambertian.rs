use ::scenery::hitable::HitRecord;
use ::utils::ray::Ray;
use ::utils::vec3::Vec3;
use super::{Material,Scatterable};

pub struct Lambertian {
    pub albebo: Vec3
}

impl Scatterable for Lambertian {
    fn scatter(&self, _ray: &Ray, hit_record: &HitRecord) -> Option<Material> {
        let target = hit_record.normal + random_in_unit_sphere();

        Some(Material {
            ray: Ray::new(hit_record.p, target),
            attenuation: self.albebo
        })
    }
}

fn random_in_unit_sphere() -> Vec3 {
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
