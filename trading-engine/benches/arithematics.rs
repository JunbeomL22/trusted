use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_addition(c: &mut Criterion) {
    let mut group = c.benchmark_group("100 addition");
    let vec_i32: Vec<i32> = (1..=100).collect();
    group.bench_function("addition i32", |b| b.iter(|| {
        let mut sum = 0;
        for i in vec_i32.iter() {
            sum += i;
        }
        sum
    }));

    let vec_i64: Vec<i64> = (1..=100).collect();
    group.bench_function("addition i64", |b| b.iter(|| {
        let mut sum = 0;
        for i in vec_i64.iter() {
            sum += i;
        }
        sum
    }));

    let vec_u32: Vec<u32> = (1..=100).collect();
    group.bench_function("addition u32", |b| b.iter(|| {
        let mut sum = 0;
        for i in vec_u32.iter() {
            sum += i;
        }
        sum
    }));

    let vec_u64: Vec<u64> = (1..=100).collect();
    group.bench_function("addition u64", |b| b.iter(|| {
        let mut sum = 0;
        for i in vec_u64.iter() {
            sum += i;
        }
        sum
    }));

    let vec_f32: Vec<f32> = (1..=100).map(|x| x as f32).collect();
    group.bench_function("addition f32", |b| b.iter(|| {
        let mut sum = 0.0;
        for i in vec_f32.iter() {
            sum += i;
        }
        sum
    }));

    let vec_f64: Vec<f64> = (1..=100).map(|x| x as f64).collect();
    group.bench_function("addition f64", |b| b.iter(|| {
        let mut sum = 0.0;
        for i in vec_f64.iter() {
            sum += i;
        }
        sum
    }));

    group.finish();
}

fn bench_multiply(c: &mut Criterion) {
    let mut group = c.benchmark_group("100 multiplication");
    let vec_i32: Vec<i32> = (1..=100).collect();
    group.bench_function("multiplication i32", |b| b.iter(|| {
        let mut product = 1;
        for i in vec_i32.iter() {
            product *= i;
        }
        product
    }));

    let vec_i64: Vec<i64> = (1..=100).collect();
    group.bench_function("multiplication i64", |b| b.iter(|| {
        let mut product = 1;
        for i in vec_i64.iter() {
            product *= i;
        }
        product
    }));

    let vec_u32: Vec<u32> = (1..=100).collect();
    group.bench_function("multiplication u32", |b| b.iter(|| {
        let mut product = 1;
        for i in vec_u32.iter() {
            product *= i;
        }
        product
    }));

    let vec_u64: Vec<u64> = (1..=100).collect();
    group.bench_function("multiplication u64", |b| b.iter(|| {
        let mut product = 1;
        for i in vec_u64.iter() {
            product *= i;
        }
        product
    }));

    let vec_f32: Vec<f32> = (1..=100).map(|x| x as f32).collect();
    group.bench_function("multiplication f32", |b| b.iter(|| {
        let mut product = 1.0;
        for i in vec_f32.iter() {
            product *= i;
        }
        product
    }));

    let vec_f64: Vec<f64> = (1..=100).map(|x| x as f64).collect();
    group.bench_function("multiplication f64", |b| b.iter(|| {
        let mut product = 1.0;
        for i in vec_f64.iter() {
            product *= i;
        }
        product
    }));

    group.finish();
}

fn bench_division(c: &mut Criterion) {
    let mut group = c.benchmark_group("100 division");
    let vec_i32: Vec<i32> = (1..=100).collect();
    group.bench_function("division i32", |b| b.iter(|| {
        let mut quotient = 1;
        for i in vec_i32.iter() {
            quotient /= i;
        }
        quotient
    }));

    let vec_i64: Vec<i64> = (1..=100).collect();
    group.bench_function("division i64", |b| b.iter(|| {
        let mut quotient = 1;
        for i in vec_i64.iter() {
            quotient /= i;
        }
        quotient
    }));

    let vec_u32: Vec<u32> = (1..=100).collect();
    group.bench_function("division u32", |b| b.iter(|| {
        let mut quotient = 1;
        for i in vec_u32.iter() {
            quotient /= i;
        }
        quotient
    }));

    let vec_u64: Vec<u64> = (1..=100).collect();
    group.bench_function("division u64", |b| b.iter(|| {
        let mut quotient = 1;
        for i in vec_u64.iter() {
            quotient /= i;
        }
        quotient
    }));

    let vec_f32: Vec<f32> = (1..=100).map(|x| x as f32).collect();
    group.bench_function("division f32", |b| b.iter(|| {
        let mut quotient = 1.0;
        for i in vec_f32.iter() {
            quotient /= i;
        }
        quotient
    }));

    let vec_f64: Vec<f64> = (1..=100).map(|x| x as f64).collect();
    group.bench_function("division f64", |b| b.iter(|| {
        let mut quotient = 1.0;
        for i in vec_f64.iter() {
            quotient /= i;
        }
        quotient
    }));

    group.finish();
}

criterion_group!(
    benches, 
    bench_multiply, bench_division,
);
criterion_main!(benches);
