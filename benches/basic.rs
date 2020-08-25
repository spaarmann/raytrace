use criterion::{criterion_group, criterion_main, Criterion};
use raytrace::*;
use std::time::Duration;

fn load_scene(path: &str) -> Scene {
    let file_content = std::fs::read_to_string(path).unwrap();
    deserialize_scene(&file_content).unwrap()
}

fn sphere1() {
    let scene = load_scene("benches/sphere1.scene");
    let image_settings = ImageSettings {
        width: 100,
        height: 100,
    };
    let render_settings = RenderSettings {
        samples_per_pixel: 5,
        max_depth: 10,
        thread_count: 1,
    };
    render(&scene, &image_settings, &render_settings, false);
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("spheres");
    group.sample_size(20);
    group.measurement_time(Duration::from_secs(25));
    group.bench_function("sphere1", |b| b.iter(sphere1));
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
