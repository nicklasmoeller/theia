use ::scenery::hitable::HitRecord;
use ::utils::ray::Ray;
use ::utils::vec3::Vec3;
use super::{Material, Scatterable};

pub struct Dielectric {
    pub ref_idx: f32
}

impl Scatterable for Dielectric {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<Material> {
        let outward_normal: Vec3;
        let reflected: Vec3 = reflect(ray.direction, hit_record.normal);;
        let mut refracted: Vec3 = Vec3::new(0.0, 0.0, 0.0);
        let target: Vec3;
        let ni_over_nt: f32;

        let reflect_prob: f32;
        let cosine: f32;

        if ray.direction.dot(&hit_record.normal) > 0.0 {
            outward_normal = -1.0 * hit_record.normal;
            ni_over_nt = self.ref_idx;
            cosine = self.ref_idx * ray.direction.dot(&hit_record.normal) / ray.direction.length();
        } else {
            outward_normal = hit_record.normal;
            ni_over_nt = 1.0 / self.ref_idx;
            cosine = -1.0 * self.ref_idx * ray.direction.dot(&hit_record.normal) / ray.direction.length();
        }

        if let Some(scattered) = refract(ray.direction, outward_normal, ni_over_nt) {
            refracted = scattered;
            reflect_prob = schlick(cosine, self.ref_idx);
        } else {
            reflect_prob = 1.0;
        }

        if rand::random::<f32>() < reflect_prob {
            target = reflected;
        }  else {
            target = refracted;
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

fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powf(2.0);
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}
