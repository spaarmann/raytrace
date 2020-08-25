use criterion::{criterion_group, criterion_main, Criterion};
use raytrace::*;
use std::time::Duration;

fn load_scene(path: &str) -> Scene {
    let file_content = std::fs::read_to_string(path).unwrap();
    deserialize_scene(&file_content).unwrap()
}

const SPHERE_IMAGE_SETTINGS: ImageSettings = ImageSettings {
    width: 100,
    height: 100,
};
const SPHERE_RENDER_SETTINGS: RenderSettings = RenderSettings {
    samples_per_pixel: 5,
    max_depth: 10,
    thread_count: 1,
};

fn sphere1() {
    let scene = load_scene("benches/sphere1.scene");
    render(
        &scene,
        &SPHERE_IMAGE_SETTINGS,
        &SPHERE_RENDER_SETTINGS,
        false,
    );
}

fn sphere2() {
    let scene = load_scene("benches/sphere2.scene");
    render(
        &scene,
        &SPHERE_IMAGE_SETTINGS,
        &SPHERE_RENDER_SETTINGS,
        false,
    );
}

fn sphere3() {
    let scene = load_scene("benches/sphere3.scene");
    render(
        &scene,
        &SPHERE_IMAGE_SETTINGS,
        &SPHERE_RENDER_SETTINGS,
        false,
    );
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("spheres");
    group.sample_size(20);
    group.measurement_time(Duration::from_secs(25));
    group.bench_function("sphere1", |b| b.iter(sphere1));
    group.bench_function("sphere2", |b| b.iter(sphere2));
    group.bench_function("sphere3", |b| b.iter(sphere3));
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
