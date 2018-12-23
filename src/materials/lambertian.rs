use ::scenery::hitable::HitRecord;
use ::utils::ray::Ray;
use ::utils::vec3::Vec3;
use super::{Material, Scatterable, random_in_unit_sphere};

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
