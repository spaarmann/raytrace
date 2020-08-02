use rand::Rng;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

mod camera;
mod hit;
mod ray;
mod vec3;

use camera::Camera;
use hit::{Hittable, HittableList, Sphere};
use ray::Ray;
use vec3::Vec3;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 400;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
const SAMPLES: u32 = 100;

fn ray_color(ray: Ray, scene: &dyn Hittable) -> Vec3 {
    match scene.hit(ray, 0.0..100.0) {
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

    let mut rng = rand::thread_rng();

    // Camera
    let camera = Camera::new(ASPECT_RATIO, 2.0, 1.0, Vec3::ZERO);

    // Scene
    let s1 = Sphere {
        center: Vec3(0.0, 0.0, -1.0),
        radius: 0.5,
    };
    let s2 = Sphere {
        center: Vec3(0.0, -100.5, -1.0),
        radius: 100.0,
    };
    let scene = HittableList {
        hittables: vec![&s1, &s2],
    };

    for j in (0..IMAGE_HEIGHT).rev() {
        println!("Scanline {}/{}", IMAGE_HEIGHT - j, IMAGE_HEIGHT);
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Vec3::ZERO;
            for _ in 0..SAMPLES {
                let u = (f64::from(i) + rng.gen::<f64>()) / f64::from(IMAGE_WIDTH - 1);
                let v = (f64::from(j) + rng.gen::<f64>()) / f64::from(IMAGE_HEIGHT - 1);

                let ray = camera.get_ray(u, v);
                pixel_color += ray_color(ray, &scene);
            }

            write_color(&mut pixels, pixel_color / (SAMPLES as f64));
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
