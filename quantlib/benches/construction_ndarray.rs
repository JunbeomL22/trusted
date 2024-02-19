use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use ndarray::Array1;

fn vec_construction(n: i32) -> Vec<f32> {
    (0..n).map(|x| x as f32).collect()
}
    

fn array_construction_by_into(n: i32) -> Array1<f32> {
    (0..n).map(|x| x as f32).collect::<Vec<f32>>().into()
}

fn array_construction_from_range(n: f32) -> Array1<f32> {
    Array1::range(0.0, n, 1.0)
}

fn create_vector_then_conversion(n: i32) -> Array1<f32> {
    let vec = (0..n).map(|x| x as f32).collect();
    Array1::from_vec(vec)
}

fn vec_to_array_conversion(vec: Vec<f32>) -> Array1<f32> {
    Array1::from_vec(vec)
}

fn benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("Construction Benchmarks");

    for n in [1000000].iter() {
        group.bench_with_input(BenchmarkId::new("vec_construction", n), n, |b, &n| {
            b.iter(|| vec_construction(n));
        });
        
        group.bench_with_input(BenchmarkId::new("array_construction_by_into", n), n, |b, &n| {
            b.iter(|| array_construction_by_into(n));
        });

        let m = *n as f32;
        group.bench_with_input(BenchmarkId::new("array_construction_from_range", m), &m, |b, &_m| {
            b.iter(|| array_construction_from_range(m));
        });

        group.bench_with_input(BenchmarkId::new("create_vector_then_conversion", n), n, |b, &n| {
            b.iter(|| create_vector_then_conversion(n));
        });

        group.bench_with_input(BenchmarkId::new("vec_to_array_conversion", n), n, |b, &n| {
            let vec: Vec<f32> = (0..n).map(|x| x as f32).collect();
            b.iter(|| vec_to_array_conversion(vec.clone()));
        });
    }
    group.finish();
}

criterion_group!(benches, benchmarks);
criterion_main!(benches);