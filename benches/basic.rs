use criterion::{criterion_group, criterion_main, Criterion};
use raytrace::*;

fn load_scene(path: &str) -> (Box<dyn Hittable>, Camera) {
    let file_content = std::fs::read_to_string(path).unwrap();
    deserialize_scene(&file_content).unwrap()
}

fn sphere1() {
    let (scene, camera) = load_scene("benches/sphere1.scene");
    render(scene.as_ref(), &camera, 100, 100, 10, 5, 1, false);
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("sphere1", |b| b.iter(sphere1));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
