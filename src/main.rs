extern crate lodepng;

mod utils;

use utils::rgb::RGB;

fn main() {
    const WIDTH: usize = 800;
    const HEIGHT: usize = 400;

    let mut data = [RGB::default(); WIDTH * HEIGHT];

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let r_point = x as f32 / WIDTH as f32;
            let g_point = y as f32 / HEIGHT as f32;
            let b_point = 0.2f32;
            let r = (255.99 * r_point) as u8;
            let g = (255.99 * g_point) as u8;
            let b = (255.99 * b_point) as u8;
            data[x + (HEIGHT - y - 1) * WIDTH] = RGB { r, g, b };
        }
    }

    match lodepng::encode24_file("output.png", &data, WIDTH, HEIGHT) {
        Ok(_) => println!("Rendered image to output.png"),
        Err(e) => println!("{}", e)
    }
}
