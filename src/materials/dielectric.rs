use ::scenery::hitable::HitRecord;
use ::utils::ray::Ray;
use ::utils::vec3::Vec3;
use super::{Material, Scatterable};

pub struct Dielectric {
    pub ri: f32
}

impl Scatterable for Dielectric {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<Material> {
        let outward_normal: Vec3;
        let target: Vec3;
        let ni_over_nt: f32;

        if ray.direction.dot(&hit_record.normal) > 0.0 {
            outward_normal = -1.0 * hit_record.normal;
            ni_over_nt = self.ri;
        } else {
            outward_normal = hit_record.normal;
            ni_over_nt = 1.0 / self.ri;
        }

        if let Some(scattered) = refract(ray.direction, outward_normal, ni_over_nt) {
            target = scattered;
        } else {
            target = reflect(ray.direction, hit_record.normal);
        }

        Some(Material {
            ray: Ray::new(hit_record.p, target),
            attenuation: Vec3::new(1.0, 1.0, 1.0)
        })
    }
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * v.dot(&n) * n
}

fn refract(v: Vec3, n: Vec3, ni_over_nt: f32) -> Option<Vec3> {
    let uv: Vec3 = Vec3::unit_vector(v);
    let dt: f32 = uv.dot(&n);
    let discriminant: f32 = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        Some(ni_over_nt * (uv - n * dt) - n * discriminant.sqrt())
    } else {
        None
    }
}
