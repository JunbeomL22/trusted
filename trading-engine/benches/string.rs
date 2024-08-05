use criterion::{criterion_group, criterion_main, Criterion, black_box};
use flexstr::{IntoSharedStr, LocalStr, SharedStr};
use std::str::{from_utf8, from_utf8_unchecked};

fn bench_string_creation(c: &mut Criterion) {
    let str_data = "KR0123456789";
    let mut group = c.benchmark_group("String Creation 1000 time");
    group.bench_function("std::String::from", |b| {
        b.iter(|| {
            for _ in 0..1000 {
                let _ = String::from(str_data);
            }
        })
    });

    group.bench_function("LocalStr::from_str", |b| {
        b.iter(|| {
            for _ in 0..1000 {
                let _ = LocalStr::from(str_data);
            }
        })
    });

    group.bench_function("SharedStr::from_str", |b| {
        b.iter(|| {
            for _ in 0..1000 {
                let _ = SharedStr::from(str_data);
            }
        })
    });

    group.bench_function("Local to Shared", |b| {
        let s = LocalStr::from(str_data);
        b.iter(|| {
            for _ in 0..1_000 {
                let _: SharedStr = SharedStr::from(s.as_str());
            }
        })
    });

    group.finish();
}

fn bench_comparison(c: &mut Criterion) {
    let str_data = "KR0123456789";
    let mut group = c.benchmark_group("String Comparison 1_000 time");
    let s1 = String::from(str_data);
    let s3 = LocalStr::from(str_data);
    let s4 = SharedStr::from(str_data);

    let str_data_vec = (0..1000).map(|_| str_data.clone()).collect::<Vec<&str>>();
    let s1_vec = (0..1000).map(|_| s1.clone()).collect::<Vec<String>>();
    let s3_vec = (0..1000).map(|_| s3.clone()).collect::<Vec<LocalStr>>();
    let s4_vec = (0..1000).map(|_| s4.clone()).collect::<Vec<SharedStr>>();
    
    group.bench_function("std::String::eq", |b| {
        b.iter(|| {
            for i in 0..1_000 {
                let _ = s1_vec[i] == black_box(str_data_vec[i]);
            }
        })
    });

    group.bench_function("LocalStr::eq", |b| {
        b.iter(|| {
            for i in 0..1_000 {
                let _ = s3_vec[i] == black_box(str_data_vec[i]);
            }
        })
    });

    group.bench_function("SharedStr::eq", |b| {
        b.iter(|| {
            for i in 0..1_000 {
                let _ = s4_vec[i] == black_box(str_data_vec[i]);
            }
        })
    });

    group.finish();
}

fn bench_byte_conversions(c: &mut Criterion) {
    let mut group = c.benchmark_group("&[u8] to &str 1_000_000");
    let byte_data = b"B604F        G140KR41EYV1000900421009135225519800028350000028300000000002800000006500006000190002840000002825000000000810000001190001000018000284500000282000000000115000000113000130002400028500000028150000000008900000011600014000140002855000002810000000000710000000660002300016000286000000280500000000072000000117000180001300028650000028000000000005200000007900012000160002870000002795000000000460000000310001600009000287500000279000000000048000000066000130001000028800000027850000000003400000005600014000060000014850000011550033400305000000000";
    let str_data = "B604F        G140KR41EYV1000900421009135225519800028350000028300000000002800000006500006000190002840000002825000000000810000001190001000018000284500000282000000000115000000113000130002400028500000028150000000008900000011600014000140002855000002810000000000710000000660002300016000286000000280500000000072000000117000180001300028650000028000000000005200000007900012000160002870000002795000000000460000000310001600009000287500000279000000000048000000066000130001000028800000027850000000003400000005600014000060000014850000011550033400305000000000";

    group.bench_function("(safe) from_utf8", |b| {
        b.iter(|| {
            for _ in 0..1_000_000 {
                let _ = from_utf8(byte_data).unwrap();
            }
        })
    });

    group.bench_function("(unsafe) from_utf8_unchecked", |b| {
        b.iter(|| unsafe {
            for _ in 0..1_000_000 {
                let _ = from_utf8_unchecked(byte_data);
            }
        })
    });

    group.bench_function("(small-bolck unsafe) from_utf8_unchecked", |b| {
        b.iter(|| {
            for _ in 0..1_000_000 {
                let _ = unsafe { from_utf8_unchecked(byte_data) };
            }
        })
    });

    group.bench_function("str to byte", |b| {
        b.iter(|| {
            for _ in 0..1_000_000 {
                let _ = str_data.as_bytes();
            }
        })
    });

    group.finish();
}

fn bench_conversion(c: &mut Criterion) {
    let mut group = c.benchmark_group("String Conversion 1_000_000 time");

    let string_data = String::from("0123456789");
    group.bench_function("String to str by as_str", |b| {
        let u64_str = "123456789";
        b.iter(|| {
            for _ in 0..1_000_000 {
                let _ = string_data.as_str();
            }
        });
    });
    group.bench_function("String to str by deref", |b| {
        let u64_str = "123456789";
        b.iter(|| {
            for _ in 0..1_000_000 {
                let _ = &*string_data;
            }
        });
    });

    group.bench_function("String to byte", |b| {
        let u64_str = "123456789";
        b.iter(|| {
            for _ in 0..1_000_000 {
                let _ = string_data.as_bytes();
            }
        });
    });

    let localstr_data = LocalStr::from("0123456789");
    group.bench_function("LocalStr to str by as_str", |b| {
        b.iter(|| {
            for _ in 0..1_000_000 {
                let _ = localstr_data.as_str();
            }
        });
    });

    group.bench_function("LocalStr to byte", |b| {
        b.iter(|| {
            for _ in 0..1_000_000 {
                let _ = localstr_data.as_bytes();
            }
        });
    });

    group.finish();
}
criterion_group!(
    benches,
    bench_comparison,
    bench_conversion,
    bench_byte_conversions,
    bench_string_creation,
);
criterion_main!(benches);
