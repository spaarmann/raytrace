use std::env;
use std::error::Error;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

mod hit;
mod ray;
mod vec3;

use hit::{Hittable, Sphere};
use ray::Ray;
use vec3::Vec3;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 400;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;

fn ray_color(ray: Ray) -> Vec3 {
    let sphere = Sphere {
        center: Vec3(0.0, 0.0, -1.0),
        radius: 0.5,
    };

    match sphere.hit(ray, 0.0, 100.0) {
        Some(hit) => 0.5 * (hit.normal + 1.0),
        None => {
            let t = 0.5 * (ray.direction.normalized().1 + 1.0);
            (1.0 - t) * Vec3::ONE + t * Vec3(0.5, 0.7, 1.0)
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Output file
    let args: Vec<String> = env::args().collect();
    let path = Path::new(&args[1]);
    let file = BufWriter::new(File::create(&path).unwrap());
    let mut encoder = png::Encoder::new(file, IMAGE_WIDTH, IMAGE_HEIGHT);
    encoder.set_color(png::ColorType::RGB);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header()?;

    let mut pixels = Vec::<u8>::with_capacity((IMAGE_WIDTH * IMAGE_HEIGHT * 4) as usize);

    // Camera
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3::ZERO;
    let horizontal = Vec3(viewport_width, 0.0, 0.0);
    let vertical = Vec3(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3(0.0, 0.0, focal_length);

    for j in (0..IMAGE_HEIGHT).rev() {
        println!("Scanline {}/{}", IMAGE_HEIGHT - j, IMAGE_HEIGHT);
        for i in 0..IMAGE_WIDTH {
            let u = f64::from(i) / f64::from(IMAGE_WIDTH - 1);
            let v = f64::from(j) / f64::from(IMAGE_HEIGHT - 1);

            let ray = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            let pixel_color = ray_color(ray);

            write_color(&mut pixels, pixel_color);
        }
    }

    writer.write_image_data(&pixels)?;

    println!("Done.");
    Ok(())
}

fn write_color(pixels: &mut Vec<u8>, color: Vec3) {
    let color = 255.999 * color; // Map from [0-1] to [0-255]
    pixels.push(color.0 as u8);
    pixels.push(color.1 as u8);
    pixels.push(color.2 as u8);
}
