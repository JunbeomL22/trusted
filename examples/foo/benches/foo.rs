use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput, BenchmarkId};
use std::simd::f32x4;

use std::time::Duration;

// function that find an index of a value in a vector
fn find_index<T: PartialEq>(vec: Vec<T>, value: T) -> Option<usize> {
    for (i, item) in vec.iter().enumerate() {
        if *item == value {
            return Some(i);
        }
    }
    None
}

// multiply the input vetor by 2.0 for each element
fn multiply_by_two(vec: Vec<f32>) -> Vec<f32> {
    vec.iter().map(|x| x * 2.0).collect()
}

// multiply the input vetor by 2.0 for each element by using WideF32x4
fn multiply_by_two_simd(vec: Vec<WideF32x4>) -> Vec<WideF32x4> {
    vec.iter().map(|x| *x * WideF32x4::splat(2.0)).collect();
}



fn benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("multiply_by_two");
    group.measurement_time(Duration::new(2, 0));

    let n: u64 = 10_000_000;
    // benchmark 100,000,000 vector of integers
    //multiply_by_two
    let vec: Vec<f32> = (0..n).map(|x| x as f32).collect();
    group.bench_function("multiply_by_two", |b| b.iter(|| multiply_by_two(black_box(vec.clone()))));
}

criterion_group!(benches, benchmark);
criterion_main!(benches);