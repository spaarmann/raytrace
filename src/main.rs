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
const IMAGE_WIDTH: u32 = 1920;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
const MAX_DEPTH: i32 = 50;

const THREAD_COUNT: u32 = 10;
const SAMPLES_PER_THREAD: u32 = 10;

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

    let mut objects: Vec<Box<dyn Hittable + Sync>> = Vec::new();

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

            if random_mat < 0.75 {
                // diffuse
                let albedo = Vec3(rng.gen(), rng.gen(), rng.gen());
                objects.push(Box::new(Sphere {
                    center,
                    radius: 0.2,
                    material: Box::new(Lambertian {
                        albedo: albedo * albedo,
                    }),
                }));
            } else if random_mat < 0.9 {
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
        0.05,
        4.0,
    );

    let scene = HittableList { hittables: objects };

    (scene, camera)
}

fn render(scene: &dyn Hittable, camera: &Camera) -> Vec<Vec3> {
    let mut rng = rand::thread_rng();
    let mut pixels = Vec::with_capacity((IMAGE_WIDTH * IMAGE_HEIGHT) as usize);

    for j in (0..IMAGE_HEIGHT).rev() {
        println!("Scanline {}/{}", IMAGE_HEIGHT - j, IMAGE_HEIGHT);
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Vec3::ZERO;
            for _ in 0..SAMPLES_PER_THREAD {
                let u = (f64::from(i) + rng.gen::<f64>()) / f64::from(IMAGE_WIDTH - 1);
                let v = (f64::from(j) + rng.gen::<f64>()) / f64::from(IMAGE_HEIGHT - 1);

                let ray = camera.get_ray(u, v);
                pixel_color += ray_color(ray, scene, MAX_DEPTH);
            }

            pixels.push(pixel_color);
        }
    }

    pixels
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

    let (scene, camera) = random_scene();

    let png_pixels = crossbeam_utils::thread::scope(|s| {
        // Start THREAD_COUNT threads, each rendering the scene.
        let mut thread_results = Vec::with_capacity(THREAD_COUNT as usize);
        for _ in 0..THREAD_COUNT {
            thread_results.push(s.spawn(|_| render(&scene, &camera)));
        }

        // Accumulate all the results into one buffer.
        // (We re-use the buffer of one of the threads, just to avoid allocating another one.)
        let mut result_pixels = thread_results.pop().unwrap().join().unwrap();
        for thread_pixels in thread_results.into_iter() {
            let thread_pixels = thread_pixels.join().unwrap();
            for i in 0..thread_pixels.len() {
                result_pixels[i] += thread_pixels[i];
            }
        }

        // Divide the accumulated colors by the amount of samples, and convert to 0-255 u8 color values.
        result_pixels
            .into_iter()
            .flat_map(|c| c / ((SAMPLES_PER_THREAD * THREAD_COUNT) as f64))
            .map(|c| (255.0 * (clamp(c, 0.0, 0.999))) as u8)
            .collect::<Vec<_>>()
    })
    .unwrap();

    writer.write_image_data(&png_pixels)?;

    println!("Done.");
    Ok(())
}

// f64::clamp is... not a thing, and who knows when it will be :(
// https://github.com/rust-lang/rust/issues/44095
fn clamp(x: f64, min: f64, max: f64) -> f64 {
    x.min(max).max(min)
}
