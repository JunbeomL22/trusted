use criterion::{black_box, criterion_group, criterion_main, Criterion};

// simd exp function uwing WideF32x4
fn simd_exp(x: Vec<Realx4>) -> Vec<Realx4> {
    x.iter().map(|&x| x.exp()).collect()
}
// plain exp function
fn my_exp(x: Vec<Real>) -> Vec<Real> {
    x.iter().map(|&x| x.exp()).collect()
}

fn benchmark(c: &mut Criterion) {
    let vec: Vec<Real> = (0..2000).map(|x| x as Real).collect();
    let vecx4: Vec<Realx4> = vec.chunks_exact(4).map(|x| WideF32x4::from_slice_unaligned(x)).collect();
    c.bench_function("simd_exp", |b| {
        b.iter(|| simd_exp(black_box(vecx4.clone())))
    });
    c.bench_function("plain_exp", |b| {
        b.iter(|| my_exp(black_box(vec.clone())))
    });
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
