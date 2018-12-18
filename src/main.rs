extern crate lodepng;

mod utils;

use utils::ray::Ray;
use utils::rgb::RGB;
use utils::vec3::Vec3;

fn color(ray: Ray) -> Vec3 {
    let unit_direction: Vec3 = Vec3::unit_vector(ray.direction);
    let t: f32 = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}

fn main() {
    const WIDTH: usize = 800;
    const HEIGHT: usize = 400;

    const LOWER_LEFT_CORNER: Vec3 = Vec3 {
        x: -2.0,
        y: -1.0,
        z: -1.0
    };

    const HORIZONTAL: Vec3 = Vec3 {
        x: 4.0,
        y: 0.0,
        z: 0.0
    };

    const VERTICAL: Vec3 = Vec3 {
        x: 0.0,
        y: 2.0,
        z: 0.0
    };

    const ORIGIN: Vec3 = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0
    };

    let mut data = [RGB::default(); WIDTH * HEIGHT];

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let u = x as f32 / WIDTH as f32;
            let v = y as f32 / HEIGHT as f32;
            let ray = Ray::new(ORIGIN, LOWER_LEFT_CORNER + u * HORIZONTAL + v * VERTICAL);

            let color = color(ray);
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
