use super::ray::Ray;
use super::vec3::Vec3;

pub struct Camera {
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3
}

impl Camera {
    pub fn new(look_from: Vec3, look_at: Vec3, view_up: Vec3, field_of_view: f32, aspect: f32) -> Camera {

        let theta = field_of_view * std::f32::consts::PI / 180.0;
        let half_height = (theta / 2.0 as f32).tan();
        let half_width = aspect * half_height;

        let w = Vec3::unit_vector( look_from - look_at);
        let u = Vec3::unit_vector(view_up.cross(&w));
        let v = w.cross(&u);

        Camera {
            lower_left_corner: look_from - half_width * u - half_height * v - w,
            horizontal: 2.0 * half_width * u,
            vertical: 2.0 * half_height * v,
            origin: look_from
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin
        )
    }
}
