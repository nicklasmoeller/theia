extern crate lodepng;
extern crate rand;

mod materials;
mod scenery;
mod utils;

use materials::dielectric::Dielectric;
use materials::lambertian::Lambertian;
use materials::metal::Metal;

use scenery::hitable::Hitable;
use scenery::scene::Scene;
use scenery::sphere::Sphere;

use utils::camera::Camera;
use utils::ray::Ray;
use utils::rgb::RGB;
use utils::vec3::Vec3;

fn get_color_for_ray(ray: &Ray, scene: &Scene, depth: u8) -> Vec3 {
    if let Some(hit) = scene.hit(&ray, 0.001, std::f32::MAX) {
        match hit.material.scatter(&ray, &hit) {
            Some(scatter) => {
                if depth < 50 {
                    return scatter.attenuation * get_color_for_ray(&scatter.ray, &scene, depth + 1);
                }
                return Vec3::new(0.0, 0.0, 0.0)
            },
            _ => Vec3::new(0.0, 0.0, 0.0)
        }
    } else {
        let unit_direction: Vec3 = Vec3::unit_vector(ray.direction);
        let t: f32 = 0.5 * (unit_direction.y + 1.0);
        return (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0);
    }
}

fn main() {
    const WIDTH: usize = 800;
    const HEIGHT: usize = 400;
    const SAMPLES: usize = 100;

    let mut data = [RGB::default(); WIDTH * HEIGHT];

    let mut scene: Scene = Scene::new();
    scene.add_sphere(Sphere::new(
        Vec3::new(0.0, 0.0, -1.0),
        0.5,
        Box::new(Lambertian {
            albebo: Vec3::new(0.1, 0.2, 0.5)
        })
    ));
    scene.add_sphere(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        Box::new(Lambertian {
            albebo: Vec3::new(0.8, 0.8, 0.0)
        })
    ));
    scene.add_sphere(Sphere::new(
        Vec3::new(1.0, 0.0, -1.0),
        0.5,
        Box::new(Metal {
            albebo: Vec3::new(0.8, 0.6, 0.2),
            fuzz: 0.0
        })
    ));
    scene.add_sphere(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        0.5,
        Box::new( Dielectric {
            ref_idx: 1.5
        })
    ));
    scene.add_sphere(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        -0.45,
        Box::new( Dielectric {
            ref_idx: 1.5
        })
    ));

    let camera = Camera::new(
        Vec3::new(-2.0, 2.0, 1.0),
        Vec3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        90.0,
        (WIDTH as f32) / (HEIGHT as f32)
    );

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let mut color = Vec3::new(0.0, 0.0, 0.0);
            for _s in 0..SAMPLES {
                let u = (x as f32 + rand::random::<f32>()) / WIDTH as f32;
                let v = (y as f32 + rand::random::<f32>()) / HEIGHT as f32;
                let ray = camera.get_ray(u, v);

                color = color + get_color_for_ray(&ray, &scene, 0);

            }
            color = color / SAMPLES as f32;
            // Square root color for gamma correction
            color = Vec3::new(color.x.sqrt(), color.y.sqrt(), color.z.sqrt());
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
