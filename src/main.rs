use rand::Rng;
use std::error::Error;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use structopt::StructOpt;

use raytrace::*;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 480;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
const MAX_DEPTH: u32 = 50;

const THREAD_COUNT: u32 = 1;
const SAMPLES_PER_PIXEL: u32 = 10;

#[allow(dead_code)]
fn test_scene() -> (Box<dyn Hittable>, Camera) {
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

    let scene = Box::new(HittableList {
        hittables: vec![
            Box::new(ground),
            Box::new(sphere_left),
            Box::new(sphere_middle),
            Box::new(sphere_right),
        ],
    });

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
fn random_scene() -> (Box<dyn Hittable>, Camera) {
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

    let scene = Box::new(HittableList { hittables: objects });

    (scene, camera)
}

#[derive(Debug, StructOpt)]
#[structopt(name = "raytrace")]
struct Opt {
    #[structopt(long = "save-scene", parse(from_os_str))]
    save_scene: Option<PathBuf>,

    #[structopt(long = "load-scene", parse(from_os_str))]
    load_scene: Option<PathBuf>,

    #[structopt(name = "FILE", parse(from_os_str))]
    output_file: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Output file
    let opt = Opt::from_args();

    let file = BufWriter::new(File::create(&opt.output_file).unwrap());
    let mut encoder = png::Encoder::new(file, IMAGE_WIDTH, IMAGE_HEIGHT);
    encoder.set_color(png::ColorType::RGB);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header()?;

    let (scene, camera) = match opt.load_scene {
        Some(path) => {
            let file_content = std::fs::read_to_string(path)?;
            deserialize_scene(&file_content)?
        }
        None => random_scene(),
    };

    if let Some(path) = opt.save_scene {
        let mut scene_file = BufWriter::new(File::create(&path).unwrap());
        scene_file.write_all(
            &serialize_scene(scene.as_ref(), &camera)
                .unwrap()
                .into_bytes(),
        )?;
        scene_file.flush()?;
    }

    let png_pixels = render(
        scene.as_ref(),
        &camera,
        IMAGE_WIDTH,
        IMAGE_HEIGHT,
        MAX_DEPTH,
        SAMPLES_PER_PIXEL,
        THREAD_COUNT,
    );

    writer.write_image_data(&png_pixels)?;

    println!("Done.");
    Ok(())
}
