use rand::Rng;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

mod camera;
mod hit;
mod material;
mod ray;
mod vec3;

use camera::Camera;
use hit::{Hit, Hittable, HittableList, Sphere};
use material::{Dielectric, Lambertian, Material, Metal};
use ray::Ray;
use vec3::Vec3;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 400;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
const SAMPLES: u32 = 100;
const MAX_DEPTH: i32 = 50;

fn ray_color(ray: Ray, scene: &dyn Hittable, depth: i32) -> Vec3 {
    if depth <= 0 {
        return Vec3::ZERO;
    }

    match scene.hit(ray, 0.000001..f64::INFINITY) {
        Some(hit) => hit
            .material
            .scatter(&ray, &hit)
            .map_or(Vec3::ZERO, |(attenuation, scattered)| {
                attenuation * ray_color(scattered, scene, depth - 1)
            }),
        None => {
            // background
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
    let camera = Camera::new(
        Vec3(0.0, 1.5, -2.0),
        Vec3(0.0, 1.0, 0.3),
        Vec3(0.0, 0.0, 1.0),
        90.0,
        ASPECT_RATIO,
        0.1,
        5.0,
    );

    // Scene

    let m_ground = Lambertian {
        albedo: Vec3(0.3, 0.8, 0.3),
    };

    let m_sphere_left = Lambertian {
        albedo: Vec3(0.7, 0.1, 0.1),
    };
    let m_sphere_middle = Metal {
        albedo: Vec3(0.5, 0.5, 0.5),
        fuzz: 0.3,
    };
    let m_sphere_right = Dielectric {
        refraction_index: 1.5,
    };

    let ground = Sphere {
        center: Vec3(0.0, -100.5, 1.0),
        radius: 100.0,
        material: &m_ground,
    };

    let sphere_left = Sphere {
        center: Vec3(-1.0, 0.0, 1.0),
        radius: 0.5,
        material: &m_sphere_left,
    };
    let sphere_middle = Sphere {
        center: Vec3(0.0, 0.0, 2.0),
        radius: 0.5,
        material: &m_sphere_middle,
    };
    let sphere_right = Sphere {
        center: Vec3(1.0, 0.0, 1.0),
        radius: -0.5,
        material: &m_sphere_right,
    };

    let scene = HittableList {
        hittables: vec![&ground, &sphere_left, &sphere_middle, &sphere_right],
    };

    for j in (0..IMAGE_HEIGHT).rev() {
        println!("Scanline {}/{}", IMAGE_HEIGHT - j, IMAGE_HEIGHT);
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Vec3::ZERO;
            for _ in 0..SAMPLES {
                let u = (f64::from(i) + rng.gen::<f64>()) / f64::from(IMAGE_WIDTH - 1);
                let v = (f64::from(j) + rng.gen::<f64>()) / f64::from(IMAGE_HEIGHT - 1);

                let ray = camera.get_ray(u, v);
                pixel_color += ray_color(ray, &scene, MAX_DEPTH);
            }

            write_color(&mut pixels, pixel_color / (SAMPLES as f64));
        }
    }

    writer.write_image_data(&pixels)?;

    println!("Done.");
    Ok(())
}

fn write_color(pixels: &mut Vec<u8>, color: Vec3) {
    // This would be the place to do gamma correction,
    // but currently the gamma factor is encoded as PNG metadata instead.
    pixels.push((256.0 * clamp(color.0, 0.0, 0.999)) as u8);
    pixels.push((256.0 * clamp(color.1, 0.0, 0.999)) as u8);
    pixels.push((256.0 * clamp(color.2, 0.0, 0.999)) as u8);
}

// f64::clamp is... not a thing, and who knows when it will be :(
// https://github.com/rust-lang/rust/issues/44095
fn clamp(x: f64, min: f64, max: f64) -> f64 {
    x.min(max).max(min)
}
