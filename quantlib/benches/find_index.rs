// compare vectorized_search_index_for_sorted_input and binary_search_index for all elements in vec
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use quantlib::definitions::Real;
use quantlib::utils::find_index::{binary_search_index, vectorized_search_index_for_sorted_vector};
use std;
// For all elements, the function searches index by bynary_search_index
fn search_index_by_binary_search_index<T: PartialOrd + Copy>(vec: &[T], search_vec: &[T]) -> Vec<usize> {
    let length = search_vec.len();
    let mut result = vec![0; length];
    for i in 0..length {
        result[i] = binary_search_index(vec, search_vec[i]);
    }
    result
}

// For all elements, the function find index by linear search
fn search_index_by_linear_search<T: PartialOrd + Copy>(vec: &[T], search_vec: &[T]) -> Vec<usize> {
    let length = search_vec.len();
    let mut result = vec![0; length];
    for i in 0..length {
        for j in 0..vec.len() {
            if search_vec[i] <= vec[j] {
                result[i] = j;
                break;
            }
        }
    }
    result
}

fn benchmark(c: &mut Criterion) {
    let vec: Vec<Real> = (0..2000).map(|x| x as Real).collect();
    let search_vec: Vec<Real> = (1600..1800).map(|x| x as Real).step_by(2).collect();

    let mut group = c.benchmark_group("Search Index Benchmarks (vector)");

    group.sample_size(100);
    group.measurement_time(std::time::Duration::new(5, 0));

    group.bench_function("vectorized_search_index_for_sorted_vector", |b| {
        b.iter(|| vectorized_search_index_for_sorted_vector(black_box(&vec), black_box(&search_vec)))
    });

    group.bench_function("all_binary_search_index", |b| {
        b.iter(|| search_index_by_binary_search_index(black_box(&vec), black_box(&search_vec)))
    });

    group.bench_function("all_linear_search", |b| {
        b.iter(|| search_index_by_linear_search(black_box(&vec), black_box(&search_vec)))
    });

    group.finish();
}

criterion_group!(benches, benchmark);
criterion_main!(benches);