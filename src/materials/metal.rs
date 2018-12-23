use ::scenery::hitable::HitRecord;
use ::utils::ray::Ray;
use ::utils::vec3::Vec3;
use super::{Material, Scatterable, random_in_unit_sphere};

pub struct Metal {
    pub albebo: Vec3,
    pub fuzz: f32
}

impl Scatterable for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<Material> {
        let target = reflect(Vec3::unit_vector(ray.direction), hit_record.normal);

        if target.dot(&hit_record.normal) > 0.0 {
            Some(Material {
                ray: Ray::new(hit_record.p, target + self.fuzz * random_in_unit_sphere()),
                attenuation: self.albebo
            })
        } else {
            None
        }

    }
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * v.dot(&n) * n
}
