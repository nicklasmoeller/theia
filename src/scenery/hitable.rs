use ::utils::vec3::Vec3;
use ::utils::ray::Ray;

#[derive(Copy, Clone)]
pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3
}

pub trait Hitable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}
