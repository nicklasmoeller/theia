use super::ray::Ray;
use super::vec3::Vec3;

pub struct Camera {
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f32
}

impl Camera {
    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        view_up: Vec3,
        field_of_view: f32,
        aspect: f32,
        aperture: f32,
        focus_dist: f32
    ) -> Camera {
        let lens_radius = aperture / 2.0;

        let theta = field_of_view * std::f32::consts::PI / 180.0;
        let half_height = (theta / 2.0 as f32).tan();
        let half_width = aspect * half_height;

        let w = Vec3::unit_vector( look_from - look_at);
        let u = Vec3::unit_vector(view_up.cross(&w));
        let v = w.cross(&u);

        Camera {
            lower_left_corner: look_from - half_width * focus_dist * u - half_height * focus_dist * v - focus_dist * w,
            horizontal: 2.0 * half_width * focus_dist * u,
            vertical: 2.0 * half_height * focus_dist * v,
            origin: look_from,
            u,
            v,
            w,
            lens_radius
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let rd: Vec3 = self.lens_radius * random_in_unit_disk();
        let offset: Vec3 = self.u * rd.x + self.v * rd.y;
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin - offset
        )
    }
}

fn random_in_unit_disk() -> Vec3 {
    let mut point: Vec3;
    while {
        point = 2.0 * Vec3::new(
            rand::random(),
            rand::random(),
            0.0
        ) - Vec3::new(1.0, 1.0, 0.0);

        point.dot(&point) >= 1.0
    } {}
    point
}
