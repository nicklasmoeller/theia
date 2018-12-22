extern crate lodepng;

mod scenery;
mod utils;

use scenery::hitable::Hitable;
use scenery::scene::Scene;
use scenery::sphere::Sphere;

use utils::camera::Camera;
use utils::ray::Ray;
use utils::rgb::RGB;
use utils::vec3::Vec3;

fn color(ray: &Ray, scene: &Scene) -> Vec3 {
    if let Some(hit) = scene.hit(&ray, 0.0, 100.0) {
        return 0.5 * Vec3::new(
            hit.normal.x + 1.0,
            hit.normal.y + 1.0,
            hit.normal.z + 1.0
        );
    } else {
        let unit_direction: Vec3 = Vec3::unit_vector(ray.direction);
        let t: f32 = 0.5 * (unit_direction.y + 1.0);
        return (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0);
    }
}

fn main() {
    const WIDTH: usize = 800;
    const HEIGHT: usize = 400;

    let mut data = [RGB::default(); WIDTH * HEIGHT];

    let mut scene: Scene = Scene::new();
    scene.add_sphere(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5));
    scene.add_sphere(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0));

    let camera = Camera::new();

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let u = x as f32 / WIDTH as f32;
            let v = y as f32 / HEIGHT as f32;
            let ray = camera.get_ray(u, v);

            let color = color(&ray, &scene);
            let r = (255.99 * color.x) as u8;
            let g = (255.99 * color.y) as u8;
            let b = (255.99 * color.z) as u8;
            data[x + (HEIGHT - y - 1) * WIDTH] = RGB { r, g, b };
        }
    }

    match lodepng::encode24_file("output.png", &data, WIDTH, HEIGHT) {
        Ok(_) => println!("Rendered image to output.png"),
        Err(e) => println!("{}", e)
    }
}
