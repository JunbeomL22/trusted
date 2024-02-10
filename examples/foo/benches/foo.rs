use criterion::{criterion_group, Criterion};
use trading_engine;
use quantlib;
use env_logger;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("trading_engine::base::conversions::f64_to_fixed_i64", |b| b.iter(|| trading_engine::base::conversions::f64_to_fixed_i64(1.234, 3)));
    // c.bench_function("trading_engine::base::conversions::f64_to_fixed_i64", |b| b.iter(|| trading_engine::base::conversions::f64_to_fixed_i64(1.234, 3)));
    // c.bench_function("trading_engine::base::conversions::f64_to_fixed_i64", |b| b.iter(|| trading_engine::base::conversions::f64_to_fixed_i64(1.234, 3)));
    // c.bench_function("trading_engine::base::conversions::f64_to_fixed_i64", |b| b.iter(|| trading_engine::base::conversions::f64_to_fixed_i64(1.234, 3)));
    // c.bench_function("trading_engine::base::conversions::f64_to_fixed_i64", |b| b.iter(|| trading_engine::base::conversions::f64_to_fixed_i64(1.234, 3)));
    // c.bench_function("trading_engine::base::conversions::f64_to_fixed_i64", |b| b.iter(|| trading_engine::base::conversions::f64_to_fixed_i64(1.234, 3)));
    // c.bench_function("trading_engine::base::conversions::f64_to_fixed_i64", |b| b.iter(|| trading_engine::base::conversions::f64_to_fixed_i64(1.234, 3)));
    // c.bench_function("trading_engine::base::conversions::f64_to_fixed_i64", |b| b.iter(|| trading_engine::base::conversions::f64_to_fixed_i64(1.234, 3)));
    // c.bench_function("trading_engine::base::conversions::f64_to_fixed_i64", |b| b.iter(|| trading_engine::base::conversions::f64_to_fixed_i64(1.234, 3)));
}

fn main () 
{
    env_logger::init();
    let x = trading_engine::base::conversions::f64_to_fixed_i64(1.234, 3);
    println!("{:?}", x);
    println!("{:?}", trading_engine::base::conversions::FIXED_PRECISION);
}

criterion_group!(benches, criterion_benchmark);