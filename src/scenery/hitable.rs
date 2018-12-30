use ::materials::Scatterable;
use ::scenery::aabb::AABB;
use ::utils::vec3::Vec3;
use ::utils::ray::Ray;

pub struct HitRecord<'a> {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: &'a Box<dyn Scatterable>
}

pub trait Hitable: Sync {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB>;
}
