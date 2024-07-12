use criterion::{black_box, criterion_group, criterion_main, Criterion};
use smallvec::{smallvec, SmallVec};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Mock {
    a: u64,
    b: u64,
    c: u64,
}

fn bench_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("creation 10 u32");
    group.bench_function("smallvec creation", |b| {
        b.iter(|| {
            SmallVec::<[u32; 10]>::from([
                black_box(0),
                black_box(1),
                black_box(2),
                black_box(3),
                black_box(4),
                black_box(5),
                black_box(6),
                black_box(7),
                black_box(8),
                black_box(9),
            ])
        });
    });

    group.bench_function("vector creation", |b| {
        b.iter(|| {
            Vec::<u32>::from([
                black_box(0),
                black_box(1),
                black_box(2),
                black_box(3),
                black_box(4),
                black_box(5),
                black_box(6),
                black_box(7),
                black_box(8),
                black_box(9),
            ])
        });
    });
    group.finish();

    let mut group = c.benchmark_group("creation 10 Mock");
    group.bench_function("smallvec creation", |b| {
        b.iter(|| {
            for i in 0..10 {
                let x: SmallVec<[Mock; 10]> = smallvec![black_box(Mock { a: i, b: i, c: i }); 10];
            }
        });
    });

    group.bench_function("vector creation", |b| {
        b.iter(|| {
            for i in 0..10 {
                let x: Vec<Mock> = vec![black_box(Mock { a: i, b: i, c: i }); 10];
            }
        });
    });

    group.finish();
}

fn bench_push(c: &mut Criterion) {
    let mut group = c.benchmark_group("push 10 u32");
    group.bench_function("smallvec push", |b| {
        b.iter(|| {
            let mut v = SmallVec::<[u32; 10]>::new();
            for i in 0..10 {
                v.push(black_box(i));
            }
        });
    });

    group.bench_function("vector push", |b| {
        b.iter(|| {
            let mut v = Vec::<u32>::new();
            for i in 0..10 {
                v.push(black_box(i));
            }
        });
    });

    group.bench_function("push smallvec with capacity", |b| {
        b.iter(|| {
            let mut v = SmallVec::<[u32; 10]>::with_capacity(10);
            for i in 0..10 {
                v.push(black_box(i));
            }
        });
    });

    group.bench_function("push vector with capacity", |b| {
        b.iter(|| {
            let mut v = Vec::<u32>::with_capacity(10);
            for i in 0..10 {
                v.push(black_box(i));
            }
        });
    });

    group.finish();

    let mut group = c.benchmark_group("push 1024 u32");
    group.bench_function("smallvec push", |b| {
        b.iter(|| {
            let mut v = SmallVec::<[u32; 1024]>::new();
            for i in 0..1024 {
                v.push(black_box(i));
            }
        });
    });

    group.bench_function("vector push", |b| {
        b.iter(|| {
            let mut v = Vec::<u32>::new();
            for i in 0..1024 {
                v.push(black_box(i));
            }
        });
    });

    group.bench_function("push smallvec with capacity", |b| {
        b.iter(|| {
            let mut v = SmallVec::<[u32; 1024]>::with_capacity(1024);
            for i in 0..1024 {
                v.push(black_box(i));
            }
        });
    });

    group.bench_function("push vector with capacity", |b| {
        b.iter(|| {
            let mut v = Vec::<u32>::with_capacity(1024);
            for i in 0..1024 {
                v.push(black_box(i));
            }
        });
    });

    group.finish();

    let mut group = c.benchmark_group("push 10 Mock");
    group.bench_function("smallvec push", |b| {
        b.iter(|| {
            let mut v = SmallVec::<[Mock; 10]>::new();
            for i in 0..10 {
                v.push(black_box(Mock { a: i, b: i, c: i }));
            }
        });
    });

    group.bench_function("vector push", |b| {
        b.iter(|| {
            let mut v = Vec::<Mock>::new();
            for i in 0..10 {
                v.push(black_box(Mock { a: i, b: i, c: i }));
            }
        });
    });

    group.bench_function("push smallvec with capacity", |b| {
        b.iter(|| {
            let mut v = SmallVec::<[Mock; 10]>::with_capacity(10);
            for i in 0..10 {
                v.push(black_box(Mock { a: i, b: i, c: i }));
            }
        });
    });

    group.bench_function("push vector with capacity", |b| {
        b.iter(|| {
            let mut v = Vec::<Mock>::with_capacity(10);
            for i in 0..10 {
                v.push(black_box(Mock { a: i, b: i, c: i }));
            }
        });
    });

    group.finish();

    let mut group = c.benchmark_group("push 1024 Mock");
    group.bench_function("smallvec push", |b| {
        b.iter(|| {
            let mut v = SmallVec::<[Mock; 1024]>::new();
            for i in 0..1024 {
                v.push(black_box(Mock { a: i, b: i, c: i }));
            }
        });
    });

    group.bench_function("vector push", |b| {
        b.iter(|| {
            let mut v = Vec::<Mock>::new();
            for i in 0..1024 {
                v.push(black_box(Mock { a: i, b: i, c: i }));
            }
        });
    });

    group.bench_function("push smallvec with capacity", |b| {
        b.iter(|| {
            let mut v = SmallVec::<[Mock; 1024]>::with_capacity(1024);
            for i in 0..1024 {
                v.push(black_box(Mock { a: i, b: i, c: i }));
            }
        });
    });

    group.bench_function("push vector with capacity", |b| {
        b.iter(|| {
            let mut v = Vec::<Mock>::with_capacity(1024);
            for i in 0..1024 {
                v.push(black_box(Mock { a: i, b: i, c: i }));
            }
        });
    });

    group.finish();
}

fn bench_copy_element(c: &mut Criterion) {
    let mut group = c.benchmark_group("copy element 8 u8");
    let mut dest: SmallVec<[u8; 8]> = smallvec![0; 8];

    let mut src = [0; 8];
    for i in 0..8 {
        src[i] = i as u8;
    }

    group.bench_function("smallvec copy element", |b| {
        b.iter(|| {
            for i in 0..8 {
                dest[i] = black_box(src[i]);
            }
        });
    });

    let mut dest2 = vec![0; 8];
    group.bench_function("vector copy element", |b| {
        b.iter(|| {
            for i in 0..8 {
                dest2[i] = black_box(i as u8);
            }
        });
    });

    group.finish();
}
criterion_group!(
    benches,
    bench_copy_element,
    //bench_creation,
    //bench_push,
);
criterion_main!(benches);
