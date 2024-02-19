use criterion::{criterion_group, criterion_main, Criterion};
use quantlib::Real;
use ndarray::Array2;
use ndarray::prelude::*;
//use ndarray_linalg::cholesky::*;
use quantlib::utils::chlescky_factorization::cholesky_decomposition;
use rand::thread_rng;
use rand_distr::{Distribution, Normal};

fn make_path(sample_size: usize, steps: usize, correlation_matrix: Array2<Real>) -> Array3<Real> {
    let n = correlation_matrix.shape()[0];
    let mut paths = Array3::zeros((sample_size, n, steps));
    let mut rng = thread_rng();
    let normal: Normal<Real> = Normal::new(0.0, 1.0).unwrap();
    let cholesky = cholesky_decomposition(&correlation_matrix).unwrap();

    for i in 0..sample_size {
        paths
        .index_axis_mut(Axis(0), i)
        .assign(&cholesky.dot(&Array2::from_shape_fn((n, steps), |_| normal.sample(&mut rng) as Real)));
    }
    paths
}

fn bench_generate_normal_random_number_multiple_times(c: &mut Criterion) {
    let sample_size = 100000;
    let num = 3;
    let steps = 365 * 3 + 10;
    let mut group = c.benchmark_group(format!("generate_normal_random_number_{}x{}x{}", sample_size, num, steps));
    group.sample_size(10);
    //group.warm_up_time(std::time::Duration::from_secs(2)); 
    //group.measurement_time(std::time::Duration::from_secs(30)); 

    group.bench_function("generate_normal_random_number", |b| {
        b.iter(|| {
            let mut rng = thread_rng();
            let normal: Normal<Real> = Normal::new(0.0, 1.0).unwrap();
            let path: Array3<Real> = Array3::from_shape_fn((sample_size, num, steps), |_| normal.sample(&mut rng) as Real);
        })
    });
    group.finish();
}

fn bench_correlated_path(c: &mut Criterion) {
    let sample_size = 100000;
    let steps = 365 * 3 + 10;
    let corr = 0.5 as Real;
    let correlation_matrix = Array2::from_shape_vec(
        (3, 3),
        vec![1.0, corr, corr, corr, 1.0, corr, corr, corr, 1.0],
    )
    .unwrap();

    let mut group = c.benchmark_group(format!("correlated_path_{}x{}x{}", sample_size, 3, steps));
    group.sample_size(10); 
    //group.warm_up_time(std::time::Duration::from_secs(2));
    //group.measurement_time(std::time::Duration::from_secs(30)); 

    group.bench_function("correlated_path", |b| {
        b.iter(|| make_path(sample_size, steps, correlation_matrix.clone()))
    });
    group.finish();
}

criterion_group!(benches, bench_generate_normal_random_number_multiple_times, bench_correlated_path);
criterion_main!(benches);