use criterion::{criterion_group, criterion_main, Criterion};
use std::collections::HashMap;
use std::marker::PhantomData;

#[derive(Clone)]
pub struct F64 {
    pub val: f64,
}

#[derive(Clone)]
pub struct F64Pad {
    pub val: f64,
    _pad: u8,
}

#[derive(Clone)]
pub struct F32 {
    pub val: f32,
}

#[derive(Clone)]
pub struct F32Pad {
    pub val: f32,
    _pad: u8,
}

#[derive(Clone)]
pub struct MockStruct;

pub trait MockTrait {}

impl MockTrait for MockStruct {}

#[derive(Clone)]
pub struct F32Phantom<T: MockTrait> {
    pub val: f32,
    _phantom: PhantomData<T>,
}

impl<T: MockTrait> F32Phantom<T> {
    pub fn new(val: f32) -> Self {
        F32Phantom {
            val,
            _phantom: PhantomData,
        }
    }
}

#[derive(Clone)]
pub struct F64Phantom<T: MockTrait> {
    pub val: f64,
    _phantom: PhantomData<T>,
}

impl<T: MockTrait> F64Phantom<T> {
    pub fn new(val: f64) -> Self {
        F64Phantom {
            val,
            _phantom: PhantomData,
        }
    }
}

impl<T: MockTrait> std::ops::Deref for F32Phantom<T> {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.val
    }
}

impl<T: MockTrait> std::ops::Deref for F64Phantom<T> {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.val
    }
}

#[derive(Clone)]
pub enum FloatPhantom {
    F32Phantom(F32Phantom<MockStruct>),
    F64Phantom(F64Phantom<MockStruct>),
}

fn multiplcation_phantom(x: Vec<FloatPhantom>, y: Vec<FloatPhantom>) -> Vec<f64> {
    x.iter()
        .zip(y.iter())
        .map(|(a, b)| match (a, b) {
            (FloatPhantom::F32Phantom(a), FloatPhantom::F32Phantom(b)) => {
                a.val as f64 * b.val as f64
            }
            (FloatPhantom::F64Phantom(a), FloatPhantom::F64Phantom(b)) => a.val * b.val,
            _ => panic!("Mismatched types"),
        })
        .collect()
}

fn multiplication_vec_f32raw(x: Vec<f32>, y: Vec<f32>) -> Vec<f32> {
    x.iter().zip(y.iter()).map(|(a, b)| a * b).collect()
}

fn multiplication_vec_f64raw(x: Vec<f64>, y: Vec<f64>) -> Vec<f64> {
    x.iter().zip(y.iter()).map(|(a, b)| a * b).collect()
}

fn multiplication_vec_F32(x: Vec<F32>, y: Vec<F32>) -> Vec<f32> {
    x.iter().zip(y.iter()).map(|(a, b)| a.val * b.val).collect()
}

fn multiplication_vec_F64(x: Vec<F64>, y: Vec<F64>) -> Vec<f64> {
    x.iter().zip(y.iter()).map(|(a, b)| a.val * b.val).collect()
}

fn multiplication_vec_F32Pad(x: Vec<F32Pad>, y: Vec<F32Pad>) -> Vec<f32> {
    x.iter().zip(y.iter()).map(|(a, b)| a.val * b.val).collect()
}

fn multiplication_vec_F64Pad(x: Vec<F64Pad>, y: Vec<F64Pad>) -> Vec<f64> {
    x.iter().zip(y.iter()).map(|(a, b)| a.val * b.val).collect()
}

fn multiplication_vec_F32Phantom(
    x: Vec<F32Phantom<MockStruct>>,
    y: Vec<F32Phantom<MockStruct>>,
) -> Vec<f32> {
    x.iter().zip(y.iter()).map(|(a, b)| a.val * b.val).collect()
}

fn multiplication_vec_F64Phantom(
    x: Vec<F64Phantom<MockStruct>>,
    y: Vec<F64Phantom<MockStruct>>,
) -> Vec<f64> {
    x.iter().zip(y.iter()).map(|(a, b)| a.val * b.val).collect()
}

#[derive(Clone, Hash, PartialEq, Eq, Debug, PartialOrd, Ord, Copy)]
pub struct U64 {
    pub val: u64,
}

#[derive(Clone, Hash, PartialEq, Eq, Debug, PartialOrd, Ord, Copy)]
pub struct U64Pad {
    pub val: u64,
    _pad: u8,
}

fn bench_multiplication_vec(c: &mut Criterion) {
    let x: Vec<f32> = (0..1000).map(|x| x as f32).collect();
    let y: Vec<f32> = (1..1001).map(|x| x as f32).collect();

    c.bench_function("multiplication_vec_f32", |b| {
        b.iter(|| multiplication_vec_f32raw(x.clone(), y.clone()))
    });

    let x: Vec<f64> = (0..1000).map(|x| x as f64).collect();
    let y: Vec<f64> = (1..1001).map(|x| x as f64).collect();

    c.bench_function("multiplication_vec_f64", |b| {
        b.iter(|| multiplication_vec_f64raw(x.clone(), y.clone()))
    });

    let x: Vec<F32> = (0..1000).map(|x| F32 { val: x as f32 }).collect();
    let y: Vec<F32> = (1..1001).map(|x| F32 { val: x as f32 }).collect();

    c.bench_function("multiplication_vec_F32", |b| {
        b.iter(|| multiplication_vec_F32(x.clone(), y.clone()))
    });

    let x: Vec<F64> = (0..1000).map(|x| F64 { val: x as f64 }).collect();
    let y: Vec<F64> = (1..1001).map(|x| F64 { val: x as f64 }).collect();

    c.bench_function("multiplication_vec_F64", |b| {
        b.iter(|| multiplication_vec_F64(x.clone(), y.clone()))
    });

    let x: Vec<F32Pad> = (0..1000)
        .map(|x| F32Pad {
            val: x as f32,
            _pad: 0,
        })
        .collect();
    let y: Vec<F32Pad> = (1..1001)
        .map(|x| F32Pad {
            val: x as f32,
            _pad: 0,
        })
        .collect();

    c.bench_function("multiplication_vec_F32Pad", |b| {
        b.iter(|| multiplication_vec_F32Pad(x.clone(), y.clone()))
    });

    let x: Vec<F64Pad> = (0..1000)
        .map(|x| F64Pad {
            val: x as f64,
            _pad: 0,
        })
        .collect();
    let y: Vec<F64Pad> = (1..1001)
        .map(|x| F64Pad {
            val: x as f64,
            _pad: 0,
        })
        .collect();

    c.bench_function("multiplication_vec_F64Pad", |b| {
        b.iter(|| multiplication_vec_F64Pad(x.clone(), y.clone()))
    });

    let x: Vec<F32Phantom<MockStruct>> = (0..1000)
        .map(|x| F32Phantom::<MockStruct>::new(x as f32))
        .collect();
    let y: Vec<F32Phantom<MockStruct>> = (1..1001)
        .map(|x| F32Phantom::<MockStruct>::new(x as f32))
        .collect();

    c.bench_function("multiplication_vec_F32Phantom", |b| {
        b.iter(|| multiplication_vec_F32Phantom(x.clone(), y.clone()))
    });

    let x: Vec<F64Phantom<MockStruct>> = (0..1000)
        .map(|x| F64Phantom::<MockStruct>::new(x as f64))
        .collect();
    let y: Vec<F64Phantom<MockStruct>> = (1..1001)
        .map(|x| F64Phantom::<MockStruct>::new(x as f64))
        .collect();

    c.bench_function("multiplication_vec_F64Phantom", |b| {
        b.iter(|| multiplication_vec_F64Phantom(x.clone(), y.clone()))
    });

    let x: Vec<FloatPhantom> = (0..1000)
        .map(|x| FloatPhantom::F32Phantom(F32Phantom::<MockStruct>::new(x as f32)))
        .collect();
    let y: Vec<FloatPhantom> = (1..1001)
        .map(|x| FloatPhantom::F32Phantom(F32Phantom::<MockStruct>::new(x as f32)))
        .collect();

    c.bench_function("multiplcation_phantom", |b| {
        b.iter(|| multiplcation_phantom(x.clone(), y.clone()))
    });

    let x: Vec<FloatPhantom> = (0..1000)
        .map(|x| FloatPhantom::F64Phantom(F64Phantom::<MockStruct>::new(x as f64)))
        .collect();
    let y: Vec<FloatPhantom> = (1..1001)
        .map(|x| FloatPhantom::F64Phantom(F64Phantom::<MockStruct>::new(x as f64)))
        .collect();

    c.bench_function("multiplcation_phantom", |b| {
        b.iter(|| multiplcation_phantom(x.clone(), y.clone()))
    });
}

fn bench_search_performance(c: &mut Criterion) {
    let mut map: HashMap<u64, u64> = HashMap::new();
    for i in 0..10_000 {
        map.insert(i, i);
    }

    c.bench_function("search_performance u64", |b| {
        b.iter(|| {
            for i in 0..10_000 {
                map.get(&i);
            }
        })
    });

    let mut map: HashMap<U64, U64> = HashMap::new();
    for i in 0..10_000 {
        map.insert(U64 { val: i }, U64 { val: i });
    }

    c.bench_function("search_performance U64 struct", |b| {
        b.iter(|| {
            for i in 0..10_000 {
                map.get(&U64 { val: i });
            }
        })
    });

    let mut map: HashMap<U64Pad, U64Pad> = HashMap::new();
    for i in 0..10_000 {
        map.insert(U64Pad { val: i, _pad: 0 }, U64Pad { val: i, _pad: 0 });
    }

    c.bench_function("search_performance U64 pad struct", |b| {
        b.iter(|| {
            for i in 0..10_000 {
                map.get(&U64Pad { val: i, _pad: 0 });
            }
        })
    });
}

criterion_group!(benches, bench_search_performance, bench_multiplication_vec,);
criterion_main!(benches);
