extern crate lodepng;

mod utils;

use utils::rgb::RGB;
use utils::vec3::Vec3;

fn main() {
    const WIDTH: usize = 800;
    const HEIGHT: usize = 400;

    let mut data = [RGB::default(); WIDTH * HEIGHT];

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let color = Vec3::new(x as f32 / WIDTH as f32, y as f32 / HEIGHT as f32, 0.2f32);
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
