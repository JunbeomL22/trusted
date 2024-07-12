use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ndarray::Array1;
use quantlib::vectorized_search_index_for_sorted_ndarray;
use quantlib::vectorized_search_index_for_sorted_vector;
use quantlib::Real;
//use std;

fn benchmark(c: &mut Criterion) {
    let vec: Vec<Real> = (0..2000).map(|x| x as Real).collect();
    let search_vec: Vec<Real> = (1600..1800).map(|x| x as Real).step_by(2).collect();

    let mut group = c.benchmark_group("Search Index Benchmarks (ndarray)");

    //group.sample_size(100);
    //group.warm_up_time(std::time::Duration::new(3, 0));
    //group.measurement_time(std::time::Duration::new(5, 0));

    group.bench_function("vectorized_search_index_for_sorted_vector", |b| {
        b.iter(|| {
            vectorized_search_index_for_sorted_vector(black_box(&vec), black_box(&search_vec))
        })
    });

    let vec: Array1<Real> = Array1::from((0..2000).map(|x| x as Real).collect::<Vec<Real>>());
    let search_vec: Array1<Real> = Array1::from(
        (1600..1800)
            .map(|x| x as Real)
            .step_by(2)
            .collect::<Vec<Real>>(),
    );

    group.bench_function("vectorized_search_index_for_sorted_ndarray", |b| {
        b.iter(|| {
            vectorized_search_index_for_sorted_ndarray(black_box(&vec), black_box(&search_vec))
        })
    });

    group.finish();
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
