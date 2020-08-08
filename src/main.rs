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
const SAMPLES: u32 = 20;
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

#[allow(dead_code)]
fn test_scene() -> (HittableList, Camera) {
    let ground = Sphere {
        center: Vec3(0.0, -100.5, 1.0),
        radius: 100.0,
        material: Box::new(Lambertian {
            albedo: Vec3(0.3, 0.8, 0.3),
        }),
    };

    let sphere_left = Sphere {
        center: Vec3(-1.0, 0.0, 1.0),
        radius: 0.5,
        material: Box::new(Lambertian {
            albedo: Vec3(0.7, 0.1, 0.1),
        }),
    };
    let sphere_middle = Sphere {
        center: Vec3(0.0, 0.0, 2.0),
        radius: 0.5,
        material: Box::new(Metal {
            albedo: Vec3(0.5, 0.5, 0.5),
            fuzz: 0.3,
        }),
    };
    let sphere_right = Sphere {
        center: Vec3(1.0, 0.0, 1.0),
        radius: -0.5,
        material: Box::new(Dielectric {
            refraction_index: 1.5,
        }),
    };

    let scene = HittableList {
        hittables: vec![
            Box::new(ground),
            Box::new(sphere_left),
            Box::new(sphere_middle),
            Box::new(sphere_right),
        ],
    };

    let camera = Camera::new(
        Vec3(0.0, 1.5, -2.0),
        Vec3(0.0, 1.0, 0.3),
        Vec3(0.0, 0.0, 1.0),
        90.0,
        ASPECT_RATIO,
        0.1,
        5.0,
    );

    (scene, camera)
}

#[allow(dead_code)]
fn random_scene() -> (HittableList, Camera) {
    let mut rng = rand::thread_rng();

    let mut objects: Vec<Box<dyn Hittable>> = Vec::new();

    // Ground
    objects.push(Box::new(Sphere {
        center: Vec3(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Box::new(Lambertian {
            albedo: Vec3(0.5, 0.5, 0.5),
        }),
    }));

    for a in -11..=11 {
        for b in -11..=11 {
            let center = Vec3(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>(),
            );
            let random_mat = rng.gen::<f64>();

            if random_mat < 0.8 {
                // diffuse
                let albedo = Vec3(rng.gen(), rng.gen(), rng.gen());
                objects.push(Box::new(Sphere {
                    center,
                    radius: 0.2,
                    material: Box::new(Lambertian {
                        albedo: albedo * albedo,
                    }),
                }));
            } else if random_mat < 0.95 {
                // metal
                let albedo = Vec3(
                    rng.gen::<f64>() * 0.5 + 0.5,
                    rng.gen::<f64>() * 0.5 + 0.5,
                    rng.gen::<f64>() * 0.5 + 0.5,
                );
                let fuzz = rng.gen::<f64>() * 0.5;
                objects.push(Box::new(Sphere {
                    center,
                    radius: 0.2,
                    material: Box::new(Metal { albedo, fuzz }),
                }));
            } else {
                // glass
                objects.push(Box::new(Sphere {
                    center,
                    radius: 0.2,
                    material: Box::new(Dielectric {
                        refraction_index: 1.5,
                    }),
                }));
            }
        }
    }

    let forward = Vec3(0.0, -0.2, 1.0).normalized();
    let up = forward.cross(Vec3(1.0, 0.0, 0.0));

    //let up = Vec3(0.0, 1.0, 0.7);
    let camera = Camera::new(
        Vec3(0.0, 1.0, -5.0),
        up,
        //Vec3(1.0, 0.0, 0.0).cross(up),
        forward,
        30.0,
        ASPECT_RATIO,
        0.1,
        4.0,
    );

    let scene = HittableList { hittables: objects };

    (scene, camera)
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut rng = rand::thread_rng();

    // Output file
    let args: Vec<String> = env::args().collect();
    let path = Path::new(&args[1]);
    let file = BufWriter::new(File::create(&path).unwrap());
    let mut encoder = png::Encoder::new(file, IMAGE_WIDTH, IMAGE_HEIGHT);
    encoder.set_color(png::ColorType::RGB);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header()?;

    let mut pixels = [0u8; (IMAGE_WIDTH * IMAGE_HEIGHT * 3) as usize];

    let (scene, camera) = random_scene();

    for j in 0..IMAGE_HEIGHT {
        println!("Scanline {}/{}", j + 1, IMAGE_HEIGHT);
        for i in 0..IMAGE_WIDTH {
            let index = (i + (IMAGE_HEIGHT - j - 1) * IMAGE_WIDTH) * 3;
            let mut pixel_color = Vec3::ZERO;
            for _ in 0..SAMPLES {
                let u = (f64::from(i) + rng.gen::<f64>()) / f64::from(IMAGE_WIDTH - 1);
                let v = (f64::from(j) + rng.gen::<f64>()) / f64::from(IMAGE_HEIGHT - 1);

                let ray = camera.get_ray(u, v);
                pixel_color += ray_color(ray, &scene, MAX_DEPTH);
            }

            write_color(&mut pixels, index as usize, pixel_color / (SAMPLES as f64));
        }
    }

    writer.write_image_data(&pixels)?;

    println!("Done.");
    Ok(())
}

fn write_color(pixels: &mut [u8], index: usize, color: Vec3) {
    // This would be the place to do gamma correction,
    // but currently the gamma factor is encoded as PNG metadata instead.
    pixels[index] = (256.0 * clamp(color.0, 0.0, 0.999)) as u8;
    pixels[index + 1] = (256.0 * clamp(color.1, 0.0, 0.999)) as u8;
    pixels[index + 2] = (256.0 * clamp(color.2, 0.0, 0.999)) as u8;
}

// f64::clamp is... not a thing, and who knows when it will be :(
// https://github.com/rust-lang/rust/issues/44095
fn clamp(x: f64, min: f64, max: f64) -> f64 {
    x.min(max).max(min)
}
