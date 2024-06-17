use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lazy_format::{lazy_format, write};
use joinery::JoinableIterator;

fn bench_base(c: &mut Criterion) {
    let mut bgroup = c.benchmark_group("compare");

    for n in &[1, 10, 100, 1000] {
        bgroup.bench_function(format!("format/{n}"), |b| {
            b.iter(|| (0..*n)
                .map(|value| format!("\t'{}'", value))
                .join_with("\n")
                .to_string()
            );
        });
        bgroup.bench_function(format!("lazy_format/{n}"), |b| {
            b.iter(|| (0..*n)
                .map(|value| lazy_format!("\t'{}'", value))
                .join_with("\n")
                .to_string()
            );
        });
    }

    bgroup.finish();
}

fn bench_concat_numbers(c: &mut Criterion) {
    let mut bgroup = c.benchmark_group("concat_numbers");

    fn format_numbers(dest: &mut String, n: usize) {
        let values = (0..n).map(|value| format!("'{}'...", value));
        for value in values {
            write!(dest, "{}", value).unwrap();
        }
    }

    fn lazy_format_numbers(dest: &mut String, n: usize) {
        let values = (0..n).map(|value| lazy_format!("'{}'...", value));
        for value in values {
            write!(dest, "{}", value).unwrap();
        }
    }

    for n in &[1, 10, 100, 1000] {
        bgroup.bench_function(format!("format/{n}"), |b| {
            b.iter(|| {
                let mut dest = String::new();
                format_numbers(&mut dest, *n);
                dest
            });
        });

        bgroup.bench_function(format!("lazy_format/{n}"), |b| {
            b.iter(|| {
                let mut dest = String::new();
                lazy_format_numbers(&mut dest, *n);
                dest
            });
        });
    }

    bgroup.finish();
}

fn bench_datetime_formatter(c: &mut Criterion) {
    let mut bgroup = c.benchmark_group("datetime_formatter");

    let chrono_now = chrono::Utc::now();
    let time_now = time::OffsetDateTime::now_utc();

    bgroup.bench_function("chrono::DateTime -> to_string", |b| {
        b.iter(|| chrono_now.to_string());
    });

    bgroup.bench_function("chrono::DateTime -> format", |b| {
        b.iter(|| format!("{}", chrono_now));
    });

    bgroup.bench_function("time::OffsetDatetime -> to_string", |b| {
        b.iter(|| time_now.to_string());
    });

    bgroup.bench_function("time::OffsetDatetime -> format", |b| {
        b.iter(|| format!("{}", time_now));
    });

    let unix_nano = minstant::Instant::now().as_unix_nanos(&minstant::Anchor::new());
    bgroup.bench_function("unix_nano -> to_string", |b| {
        b.iter(|| {
            unix_nano.to_string();
        });
    });

    bgroup.bench_function("unix_nano -> format", |b| {
        b.iter(|| {
            format!("{}", unix_nano);
        });
    });

    bgroup.bench_function("unix_nano -> lazy_format", |b| {
        b.iter(|| {
            lazy_format!("{}", unix_nano);
        });
    });

    bgroup.finish();
}
criterion_group!(
    benches, 
    //bench_base,
    //bench_concat_numbers,
    bench_datetime_formatter,
);

criterion_main!(benches);
