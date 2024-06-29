use criterion::{criterion_group, criterion_main, Criterion};
use fixed::types::I32F32;
use ryu;
use rust_decimal::Decimal;
use rust_decimal::prelude::*;

fn dot_product_fixed(x_vec: Vec<I32F32>, y_vec: Vec<I32F32>) -> I32F32 {
    let mut sum = I32F32::from_num(0);
    for i in 0..x_vec.len() {
        sum += x_vec[i] * y_vec[i];
    }
    sum
}

fn dot_product_decimal(x_vec: Vec<Decimal>, y_vec: Vec<Decimal>) -> Decimal {
    let mut sum = Decimal::from_str("0").unwrap();
    for i in 0..x_vec.len() {
        sum += x_vec[i] * y_vec[i];
    }
    sum
}

fn dot_product_f64(x_vec: Vec<f64>, y_vec: Vec<f64>) -> f64 {
    let mut sum = 0.0;
    for i in 0..x_vec.len() {
        sum += x_vec[i] * y_vec[i];
    }
    sum
}

fn division_vector_fixed(x_vec: Vec<I32F32>, y_vec: Vec<I32F32>) -> Vec<I32F32> {
    let mut result = Vec::new();
    for i in 0..x_vec.len() {
        result.push(x_vec[i] / y_vec[i]);
    }
    result
}

fn division_vector_f64(x_vec: Vec<f64>, y_vec: Vec<f64>) -> Vec<f64> {
    let mut result = Vec::new();
    for i in 0..x_vec.len() {
        result.push(x_vec[i] / y_vec[i]);
    }
    result
}

fn division_vector_decimal(x_vec: Vec<Decimal>, y_vec: Vec<Decimal>) -> Vec<Decimal> {
    let mut result = Vec::new();
    for i in 0..x_vec.len() {
        result.push(x_vec[i] / y_vec[i]);
    }
    result
}

fn bench_division_comparison(c: &mut Criterion) {
    // make vectors
    let x: Vec<f64> = (0..1000).map(|x| x as f64).collect();
    let y: Vec<f64> = (1..1001).map(|x| x as f64).collect();

    let x_fixed: Vec<I32F32> = x.iter().map(|&x| I32F32::from_num(x)).collect();
    let y_fixed: Vec<I32F32> = y.iter().map(|&x| I32F32::from_num(x)).collect();

    let mut group = c.benchmark_group("division_comparison");
    
    group.bench_function("Decimal division", |b| b.iter(|| division_vector_decimal(x.iter().map(|&x| Decimal::from_f64(x).unwrap()).collect(), y.iter().map(|&x| Decimal::from_f64(x).unwrap()).collect())));

    group.bench_function("fixed division", |b| b.iter(|| division_vector_fixed(x_fixed.clone(), y_fixed.clone())));

    group.bench_function("f64 division", |b| b.iter(|| division_vector_f64(x.clone(), y.clone())));
    
    group.finish();
}

fn bench_dot_product_comparison(c: &mut Criterion) {
    // make vectors
    let x: Vec<f64> = (0..1000).map(|x| x as f64).collect();
    let y: Vec<f64> = (1..1001).map(|x| x as f64).collect();

    let x_fixed: Vec<I32F32> = x.iter().map(|&x| I32F32::from_num(x)).collect();
    let y_fixed: Vec<I32F32> = y.iter().map(|&x| I32F32::from_num(x)).collect();

    let mut group = c.benchmark_group("dot_product_comparison");
    
    group.bench_function("f64 dot product", |b| b.iter(|| dot_product_f64(x.clone(), y.clone())));
    
    group.bench_function("fixed dot product", |b| b.iter(|| dot_product_fixed(x_fixed.clone(), y_fixed.clone())));
    group.finish();
}

fn bench_float_to_string(c: &mut Criterion) {
    let mut buffer = ryu::Buffer::new();
    let x: f64 = 3.141592653589793;
    let x_fixed: I32F32 = I32F32::from_num(x);

    let mut group = c.benchmark_group("float_to_string");
    group.bench_function("fixed", |b| {
        b.iter(|| {
            let _ = x_fixed.to_string();
        });
    });

    group.bench_function("f64 by ryu", |b| {
        b.iter(|| {
            let printed = buffer.format(x);
        });
    });
    

    group.finish();
}

fn bench_string_to_float(c: &mut Criterion) {
    let x_str = "3.141592653589793";
    let x_fixed_str = "3.141592653589793";
    let x_decimal_str = "3.141592653589793";

    let mut group = c.benchmark_group("string_to_float");
    
    group.bench_function("Decimal::from_str", |b| {
        b.iter(|| {
            let _ = Decimal::from_str(x_fixed_str).unwrap();
        });
    });
    group.bench_function("parsing to f64 then fixed", |b| {
        b.iter(|| {
            let val_f64 = x_fixed_str.parse::<f64>().unwrap();
            let _ = I32F32::from_num(val_f64);
            //let _ = x_fixed_str.parse::<I32F32>().unwrap();
        });
    });

    group.bench_function("f64", |b| {
        b.iter(|| {
            let _ = x_str.parse::<f64>().unwrap();
        });
    });
    

    group.finish();
}

criterion_group!(
    benches, 
    bench_division_comparison,
    bench_string_to_float,
    bench_float_to_string,
    //bench_dot_product_comparison
);

criterion_main!(benches);