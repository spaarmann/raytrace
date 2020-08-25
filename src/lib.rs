pub use camera::Camera;
use hit::Hit;
pub use hit::{Hittable, HittableList, Sphere};
pub use material::{Dielectric, Lambertian, Material, Metal};
use rand::Rng;
pub use ray::Ray;
pub use vec3::Vec3;

mod camera;
mod hit;
mod material;
mod ray;
mod vec3;

pub fn serialize_scene(scene: &dyn Hittable, camera: &Camera) -> ron::Result<String> {
    ron::ser::to_string_pretty(&(scene, camera), ron::ser::PrettyConfig::new())
}

pub fn deserialize_scene(s: &str) -> ron::Result<(Box<dyn Hittable>, Camera)> {
    ron::from_str(s)
}

pub fn render(
    scene: &dyn Hittable,
    camera: &Camera,
    img_width: u32,
    img_height: u32,
    max_depth: u32,
    samples_per_pixel: u32,
    thread_count: u32,
    show_progress: bool,
) -> Vec<u8> {
    (if thread_count == 1 {
        render_thread(
            scene,
            camera,
            img_width,
            img_height,
            max_depth,
            samples_per_pixel,
            show_progress,
        )
        .into_iter()
    } else {
        crossbeam_utils::thread::scope(|s| {
            // Start THREAD_COUNT threads, each rendering the scene.
            let mut thread_results = Vec::with_capacity(thread_count as usize);
            for _ in 0..thread_count {
                thread_results.push(s.spawn(|_| {
                    render_thread(
                        scene,
                        camera,
                        img_width,
                        img_height,
                        max_depth,
                        samples_per_pixel / thread_count,
                        show_progress,
                    )
                }));
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
            result_pixels.into_iter()
        })
        .unwrap()
    })
    // Divide the accumulated colors by the amount of samples, and convert to 0-255 u8 color values.
    .flat_map(|c| c / (samples_per_pixel as f64))
    .map(|c| (255.0 * (clamp(c, 0.0, 0.999))) as u8)
    .collect::<Vec<_>>()
}

fn render_thread(
    scene: &dyn Hittable,
    camera: &Camera,
    img_width: u32,
    img_height: u32,
    max_depth: u32,
    samples_per_pixel: u32,
    show_progress: bool,
) -> Vec<Vec3> {
    let mut rng = rand::thread_rng();
    let mut pixels = Vec::with_capacity((img_width * img_height) as usize);

    for j in (0..img_height).rev() {
        if show_progress {
            println!("Scanline {}/{}", img_height - j, img_height);
        }
        for i in 0..img_width {
            let mut pixel_color = Vec3::ZERO;
            for _ in 0..samples_per_pixel {
                let u = (f64::from(i) + rng.gen::<f64>()) / f64::from(img_width - 1);
                let v = (f64::from(j) + rng.gen::<f64>()) / f64::from(img_height - 1);

                let ray = camera.get_ray(u, v);
                pixel_color += ray_color(ray, scene, max_depth);
            }

            pixels.push(pixel_color);
        }
    }

    pixels
}

fn ray_color(ray: Ray, scene: &dyn Hittable, depth: u32) -> Vec3 {
    if depth == 0 {
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

// f64::clamp is... not a thing, and who knows when it will be :(
// https://github.com/rust-lang/rust/issues/44095
fn clamp(x: f64, min: f64, max: f64) -> f64 {
    x.min(max).max(min)
}
