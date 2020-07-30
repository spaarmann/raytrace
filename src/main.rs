use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{prelude::*, BufWriter};
use std::path::Path;

mod ray;
mod vec3;

use ray::Ray;
use vec3::Vec3;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 400;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;

fn hit_sphere(center: Vec3, radius: f64, ray: Ray) -> f64 {
    let oc = ray.origin - center;
    let a = ray.direction.mag_squared();
    let half_b = oc.dot(ray.direction);
    let c = oc.mag_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;

    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - discriminant.sqrt()) / a
    }
}

fn ray_color(ray: Ray) -> Vec3 {
    let t = hit_sphere(Vec3(0.0, 0.0, -1.0), 0.5, ray);

    if t > 0.0 {
        let normal = (ray.at(t) - Vec3(0.0, 0.0, -1.0)).normalized();
        return 0.5 * (normal + 1.0);
    }

    let t = 0.5 * (ray.direction.normalized().1 + 1.0);
    (1.0 - t) * Vec3::ONE + t * Vec3(0.5, 0.7, 1.0)
}

fn main() -> Result<(), Box<dyn Error>> {
    // Output file
    let args: Vec<String> = env::args().collect();
    let path = Path::new(&args[1]);
    let mut file = BufWriter::new(File::create(&path).unwrap());

    // Camera
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3::ZERO;
    let horizontal = Vec3(viewport_width, 0.0, 0.0);
    let vertical = Vec3(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3(0.0, 0.0, focal_length);

    writeln!(&mut file, "P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT)?;

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

            write_color(&mut file, pixel_color)?;
        }
    }

    file.flush()?;

    println!("Done.");
    Ok(())
}

fn write_color<F: Write>(f: &mut F, color: Vec3) -> std::io::Result<()> {
    let color = 255.999 * color; // Map from [0-1] to [0-255]
    writeln!(
        f,
        "{} {} {}",
        color.0 as u32, color.1 as u32, color.2 as u32
    )
}
